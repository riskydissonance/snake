use rand::Rng;
use raylib::consts::ffi::KeyboardKey;
use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 450;
const SNAKE_SIZE: i32 = 10;
const FOOD_SIZE: i32 = 10;

const GRID_SQUARE_SIZE: i32 = 10;

// TODO
// - Add rendering increasing length
// - Add self-collision detection
// - Add score
// - Add speed increase
// - Add cross-over sides of screen

struct Vector2 {
    x: i32,
    y: i32,
}

impl Vector2 {
    fn new(x: i32, y: i32) -> Vector2 {
        return Vector2 { x, y };
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Food {
    position: Vector2,
    eaten: bool,
}

struct GameState {
    snake: Snake,
    food: Food,
    game_over: bool,
}

struct Snake {
    body: Vec<Vector2>,
    direction: Direction,
    speed: i32,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Snake")
        .build();

    let mut game_state = GameState {
        snake: Snake {
            body: vec![Vector2::new(0, 0)],
            direction: Direction::Down,
            speed: GRID_SQUARE_SIZE,
        },
        food: Food {
            position: Vector2::new(0, 0),
            eaten: true,
        },
        game_over: false,
    };

    // TODO probably shouldn't set speed via fps
    rl.set_target_fps(10);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if game_state.game_over || check_game_over(&game_state.snake) {
            game_state.game_over = true;
            d.draw_text("Game Over", 320, 225, 20, Color::RED);
            continue;
        }

        d.clear_background(Color::DARKGREEN);

        move_snake(&mut game_state, &d);
        draw_snake(&game_state.snake, &mut d);
        draw_food(&mut game_state, &mut d);
    }
}

fn is_in_snake(x: i32, y: i32, snake: &Snake) -> bool {
    for i in 0..snake.body.len() {
        if snake.body[i].x == x && snake.body[i].y == y {
            return true;
        }
    }
    return false;
}

fn draw_food(game_state: &mut GameState, d: &mut RaylibDrawHandle) {
    if game_state.food.eaten {
        let mut rng = rand::thread_rng();
        let mut x = ((rng.gen_range(0..WINDOW_WIDTH) + 5) / GRID_SQUARE_SIZE) * GRID_SQUARE_SIZE;
        let mut y = ((rng.gen_range(0..WINDOW_HEIGHT) + 5) / GRID_SQUARE_SIZE) * GRID_SQUARE_SIZE;
        while is_in_snake(x, y, &game_state.snake) {
            x = ((rng.gen_range(0..WINDOW_WIDTH) + 5) / GRID_SQUARE_SIZE) * GRID_SQUARE_SIZE;
            y = ((rng.gen_range(0..WINDOW_HEIGHT) + 5) / GRID_SQUARE_SIZE) * GRID_SQUARE_SIZE;
        }
        game_state.food.position.x = x;
        game_state.food.position.y = y;
        game_state.food.eaten = false;
    } else {
        if is_in_snake(
            game_state.food.position.x,
            game_state.food.position.y,
            &game_state.snake,
        ) {
            game_state.food.eaten = true;
        }
    }
    d.draw_rectangle(
        game_state.food.position.x,
        game_state.food.position.y,
        FOOD_SIZE,
        FOOD_SIZE,
        Color::BLACK,
    );
}

fn move_snake(game_state: &mut GameState, d: &RaylibDrawHandle) {
    // TODO shouldn't be able to choose to go 'backwards'
    if d.is_key_pressed(KeyboardKey::KEY_UP) {
        game_state.snake.direction = Direction::Up;
    }
    if d.is_key_pressed(KeyboardKey::KEY_DOWN) {
        game_state.snake.direction = Direction::Down;
    }
    if d.is_key_pressed(KeyboardKey::KEY_LEFT) {
        game_state.snake.direction = Direction::Left;
    }
    if d.is_key_pressed(KeyboardKey::KEY_RIGHT) {
        game_state.snake.direction = Direction::Right;
    }

    match game_state.snake.direction {
        Direction::Up => {
            game_state.snake.body.push(Vector2::new(
                game_state.snake.body.last().unwrap().x,
                game_state.snake.body.last().unwrap().y - game_state.snake.speed,
            ));
        }
        Direction::Down => {
            game_state.snake.body.push(Vector2::new(
                game_state.snake.body.last().unwrap().x,
                game_state.snake.body.last().unwrap().y + game_state.snake.speed,
            ));
        }
        Direction::Left => {
            game_state.snake.body.push(Vector2::new(
                game_state.snake.body.last().unwrap().x - game_state.snake.speed,
                game_state.snake.body.last().unwrap().y,
            ));
        }
        Direction::Right => {
            game_state.snake.body.push(Vector2::new(
                game_state.snake.body.last().unwrap().x + game_state.snake.speed,
                game_state.snake.body.last().unwrap().y,
            ));
        }
    }

    if !game_state.food.eaten {
        game_state.snake.body.remove(0);
    }
}

fn draw_snake(snake: &Snake, d: &mut RaylibDrawHandle) {
    snake.body.iter().for_each(|part| {
        d.draw_rectangle(part.x, part.y, SNAKE_SIZE, SNAKE_SIZE, Color::BLACK);
    });
}

fn check_game_over(_snake: &Snake) -> bool {
    // TODO snake should actually cross over bounds and only end if intersects with self
    // if snake.position.x < 0
    //     || snake.position.x + SNAKE_SIZE > WINDOW_WIDTH
    //     || snake.position.y < 0
    //     || snake.position.y + SNAKE_SIZE > WINDOW_HEIGHT
    // {
    //     return true;
    // }
    return false;
}
