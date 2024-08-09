use rand::Rng;
use raylib::consts::ffi::KeyboardKey;
use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 300;
const WINDOW_HEIGHT: i32 = 300;
const SNAKE_SIZE: i32 = 10;
const FOOD_SIZE: i32 = 10;

const GRID_SQUARE_SIZE: i32 = 10;

// TODO
// - Add speed increase

struct Vector2 {
    x: i32,
    y: i32,
}

impl Vector2 {
    fn new(x: i32, y: i32) -> Vector2 {
        Vector2 { x, y }
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

impl GameState {
    fn new() -> GameState {
        GameState {
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
        }
    }
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

    let mut game_state = GameState::new();
    // TODO probably shouldn't set speed via fps
    // TODO the movement feels sluggish, maybe need to up fps but keep speed the same
    rl.set_target_fps(10);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if game_state.game_over || check_game_over(&game_state.snake) {
            game_state.game_over = true;
            d.draw_text("Game Over", 100, 140, 20, Color::RED);
            if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                game_state = GameState::new();
            }
            continue;
        }

        d.clear_background(Color::DARKGREEN);

        move_snake(&mut game_state, &d);
        draw_snake(&game_state.snake, &mut d);
        draw_food(&mut game_state, &mut d);
        draw_score(game_state.snake.body.len(), &mut d);
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
    // TODO there has to be nicer way to do this
    if d.is_key_pressed(KeyboardKey::KEY_UP) {
        if let Direction::Down = game_state.snake.direction {
        } else {
            game_state.snake.direction = Direction::Up;
        }
    }
    if d.is_key_pressed(KeyboardKey::KEY_DOWN) {
        if let Direction::Up = game_state.snake.direction {
        } else {
            game_state.snake.direction = Direction::Down;
        }
    }
    if d.is_key_pressed(KeyboardKey::KEY_LEFT) {
        if let Direction::Right = game_state.snake.direction {
        } else {
            game_state.snake.direction = Direction::Left;
        }
    }
    if d.is_key_pressed(KeyboardKey::KEY_RIGHT) {
        if let Direction::Left = game_state.snake.direction {
        } else {
            game_state.snake.direction = Direction::Right;
        }
    }

    match game_state.snake.direction {
            Direction::Up => {
                let x = game_state.snake.body.last().unwrap().x;
                let mut y = game_state.snake.body.last().unwrap().y - GRID_SQUARE_SIZE;
                if y < 0 {
                    y = WINDOW_HEIGHT - GRID_SQUARE_SIZE;
                }
                game_state.snake.body.push(Vector2::new(x, y));
            }
            Direction::Down => {
                let x = game_state.snake.body.last().unwrap().x;
                let mut y = game_state.snake.body.last().unwrap().y + GRID_SQUARE_SIZE;
                if y + GRID_SQUARE_SIZE > WINDOW_HEIGHT {
                    y = 0;
                }
                game_state.snake.body.push(Vector2::new(x, y));
            }
            Direction::Left => {
                let mut x = game_state.snake.body.last().unwrap().x - GRID_SQUARE_SIZE;
                let y = game_state.snake.body.last().unwrap().y;
                if x < 0 {
                    x = WINDOW_WIDTH - GRID_SQUARE_SIZE;
                }
                game_state.snake.body.push(Vector2::new(x, y));
            }
            Direction::Right => {
                let mut x = game_state.snake.body.last().unwrap().x + GRID_SQUARE_SIZE;
                let y = game_state.snake.body.last().unwrap().y;
                if x + GRID_SQUARE_SIZE > WINDOW_WIDTH {
                    x = 0;
                }
                game_state.snake.body.push(Vector2::new(x, y));
            }
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

fn check_game_over(snake: &Snake) -> bool {
    for i in 0..snake.body.len() {
        for j in 0..snake.body.len() {
            if i == j {
                continue;
            }
            if snake.body[i].x == snake.body[j].x && snake.body[i].y == snake.body[j].y {
                return true;
            }
        }
    }
    return false;
}

fn draw_score(score: usize, d: &mut RaylibDrawHandle) {
    d.draw_text(&format!("{}", score), 10, 10, 12, Color::BLACK);
}
