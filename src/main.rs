use rand::Rng;
use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 450;
const SNAKE_SIZE: i32 = 10;
const FOOD_SIZE: i32 = 10;

// TODO
// - Add wasm
// - Add food
// - Add rendering increasing length
// - Add self-collision detection
// - Add score
// - Add speed increase
// - Add cross-over sides of screen

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Food {
    x: i32,
    y: i32,
    eaten: bool,
}

struct GameState {
    snake: Snake,
    food: Food,
}

struct Snake {
    length: i32,
    direction: Direction,
    x: i32,
    y: i32,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Snake")
        .build();

    let mut snake = Snake {
        length: 1,
        direction: Direction::Down,
        x: 0,
        y: 0,
    };

    let mut food = Food {
        x: 0,
        y: 0,
        eaten: true,
    };

    let mut game_state = GameState { snake, food };

    let mut game_over = false;
    // TODO probably shouldn't set speed via fps
    rl.set_target_fps(120);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if game_over || check_game_over(&game_state.snake) {
            game_over = true;
            d.draw_text("Game Over", 320, 225, 20, Color::RED);
            continue;
        }

        d.clear_background(Color::DARKGREEN);

        move_snake(&mut game_state.snake, &d);
        draw_snake(&game_state.snake, &mut d);
        draw_food(&mut game_state, &mut d);
    }
}

fn is_in_snake(x: i32, y: i32, snake: &Snake) -> bool {
    if x == snake.x && y == snake.y {
        return true;
    }
    return false;
}

fn draw_food(game_state: &mut GameState, d: &mut RaylibDrawHandle) {
    if game_state.food.eaten {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(0..WINDOW_WIDTH);
        let mut y = rng.gen_range(0..WINDOW_HEIGHT);
        while is_in_snake(x, y, &game_state.snake) {
            x = rng.gen_range(0..WINDOW_WIDTH);
            y = rng.gen_range(0..WINDOW_HEIGHT);
        }
        game_state.food.x = x;
        game_state.food.y = y;
        game_state.food.eaten = false;
    } else {
        if is_in_snake(game_state.food.x, game_state.food.y, &game_state.snake) {
            game_state.snake.length += 1;
            game_state.food.eaten = true;
        }
    }
    d.draw_rectangle(
        game_state.food.x,
        game_state.food.y,
        FOOD_SIZE,
        FOOD_SIZE,
        Color::BLACK,
    );
}

fn move_snake(snake: &mut Snake, d: &RaylibDrawHandle) {
    if d.is_key_pressed(KeyboardKey::KEY_UP) {
        snake.direction = Direction::Up;
    }
    if d.is_key_pressed(KeyboardKey::KEY_DOWN) {
        snake.direction = Direction::Down;
    }
    if d.is_key_pressed(KeyboardKey::KEY_LEFT) {
        snake.direction = Direction::Left;
    }
    if d.is_key_pressed(KeyboardKey::KEY_RIGHT) {
        snake.direction = Direction::Right;
    }

    match snake.direction {
        Direction::Up => {
            snake.y -= 1;
        }
        Direction::Down => {
            snake.y += 1;
        }
        Direction::Left => {
            snake.x -= 1;
        }
        Direction::Right => {
            snake.x += 1;
        }
    }
}

fn draw_snake(snake: &Snake, d: &mut RaylibDrawHandle) {
    d.draw_rectangle(snake.x, snake.y, SNAKE_SIZE, SNAKE_SIZE, Color::BLACK);
    for i in 0..snake.length {
        d.draw_rectangle(
            snake.x,
            snake.y - i * SNAKE_SIZE,
            SNAKE_SIZE,
            SNAKE_SIZE,
            Color::BLACK,
        );
    }
}

fn check_game_over(snake: &Snake) -> bool {
    if snake.x < 0
        || snake.x + SNAKE_SIZE > WINDOW_WIDTH
        || snake.y < 0
        || snake.y + SNAKE_SIZE > WINDOW_HEIGHT
    {
        return true;
    }
    return false;
}
