mod rectangle;

use rand::Rng;
use raylib::consts::ffi::KeyboardKey;
use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 300;
const WINDOW_HEIGHT: i32 = 300;
const SNAKE_SIZE: i32 = 10;
const FOOD_RADIUS: i32 = 5;

const GRID_SQUARE_SIZE: i32 = 10;
const SNAKE_BASE_SPEED: f32 = 12.;
const SNAKE_SPEED_INCREASE: f32 = 0.25;
const BORDER_OFFSET: i32 = 5;

const CONTROL_LONG: i32 = 124;
const CONTROL_SHORT: i32 = 50;

const MAX_MOVEMENT_QUEUE: usize = 10;

const TOP_TOUCH_CONTROL: rectangle::Rectangle = rectangle::Rectangle::new(88, 20, CONTROL_LONG, CONTROL_SHORT, Color::new(150, 150, 150, 50));
const LEFT_TOUCH_CONTROL: rectangle::Rectangle = rectangle::Rectangle::new(20, 88, CONTROL_SHORT, CONTROL_LONG, Color::new(150, 150, 150, 50));
const RIGHT_TOUCH_CONTROL: rectangle::Rectangle = rectangle::Rectangle::new(230, 88, CONTROL_SHORT, CONTROL_LONG, Color::new(150, 150, 150, 50));
const BOTTOM_TOUCH_CONTROL: rectangle::Rectangle = rectangle::Rectangle::new(88, 230, CONTROL_LONG, CONTROL_SHORT, Color::new(150, 150, 150, 50));

#[derive(Debug)]
struct Vector2 {
    x: i32,
    y: i32,
}

impl Vector2 {
    fn new(x: i32, y: i32) -> Vector2 {
        Vector2 { x, y }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Food {
    position: Vector2,
    eaten: bool,
    new_food: bool,
}

#[derive(Debug)]
struct GameState {
    snake: Snake,
    food: Food,
    game_over: bool,
    time_since_last_move: f32,
    direction_queue: Vec<Direction>,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            snake: Snake {
                body: vec![Vector2::new(0, 0)],
                direction: Direction::Down,
                speed: SNAKE_BASE_SPEED,
            },
            food: Food {
                position: Vector2::new(0, 0),
                eaten: true,
                new_food: false,
            },
            game_over: false,
            time_since_last_move: 0.,
            direction_queue: Vec::with_capacity(MAX_MOVEMENT_QUEUE),
        }
    }
}

#[derive(Debug)]
struct Snake {
    body: Vec<Vector2>,
    direction: Direction,
    speed: f32,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Snek")
        .build();

    if !cfg!(target_family = "wasm") {
        let logo = Image::load_image("cobra.png").expect("Could not find cobra.png icon");
        rl.set_window_icon(&logo);
    }

    let mut game_state = GameState::new();
    rl.set_target_fps(120);
    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::LIGHTGREEN);

        if game_state.game_over || check_game_over(&game_state.snake) {
            game_state.game_over = true;
            draw_snake(&game_state.snake, &mut d);
            draw_food(&mut game_state, &mut d);
            draw_score(game_state.snake.body.len() - 1, &mut d);
            draw_border(&mut d);
            d.draw_text("Game Over", 100, 140, 20, Color::RED);
            if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                game_state = GameState::new();
            }
            continue;
        }

        game_state.time_since_last_move += delta_time;

        if cfg!(target_family = "wasm") {
            if d.get_touch_point_count() == 1 {
                let touch_point = Vector2::new(d.get_touch_x(), d.get_touch_y());
                if LEFT_TOUCH_CONTROL.intersects(&touch_point) {
                    record_next_direction_change(&mut game_state, Direction::Left);
                }
                if TOP_TOUCH_CONTROL.intersects(&touch_point) {
                    record_next_direction_change(&mut game_state, Direction::Up);
                }
                if RIGHT_TOUCH_CONTROL.intersects(&touch_point) {
                    record_next_direction_change(&mut game_state, Direction::Right);
                }
                if BOTTOM_TOUCH_CONTROL.intersects(&touch_point) {
                    record_next_direction_change(&mut game_state, Direction::Down);
                }
            }
        }
        if d.is_key_pressed(KeyboardKey::KEY_UP) {
            record_next_direction_change(&mut game_state, Direction::Up);
        }
        if d.is_key_pressed(KeyboardKey::KEY_DOWN) {
            record_next_direction_change(&mut game_state, Direction::Down);
        }
        if d.is_key_pressed(KeyboardKey::KEY_LEFT) {
            record_next_direction_change(&mut game_state, Direction::Left);
        }
        if d.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            record_next_direction_change(&mut game_state, Direction::Right);
        }

        move_snake(&mut game_state);
        draw_snake(&game_state.snake, &mut d);
        draw_food(&mut game_state, &mut d);
        draw_score(game_state.snake.body.len() - 1, &mut d);
        draw_border(&mut d);
    }
}



fn draw_border(d: &mut RaylibDrawHandle) {
    d.draw_line(
        BORDER_OFFSET,
        BORDER_OFFSET,
        WINDOW_WIDTH - BORDER_OFFSET,
        BORDER_OFFSET,
        Color::BLACK,
    );
    d.draw_line(
        BORDER_OFFSET,
        BORDER_OFFSET,
        BORDER_OFFSET,
        WINDOW_HEIGHT - BORDER_OFFSET,
        Color::BLACK,
    );
    d.draw_line(
        WINDOW_WIDTH - BORDER_OFFSET,
        BORDER_OFFSET,
        WINDOW_WIDTH - BORDER_OFFSET,
        WINDOW_HEIGHT - BORDER_OFFSET,
        Color::BLACK,
    );
    d.draw_line(
        BORDER_OFFSET,
        WINDOW_HEIGHT - BORDER_OFFSET,
        WINDOW_WIDTH - BORDER_OFFSET,
        WINDOW_HEIGHT - BORDER_OFFSET,
        Color::BLACK,
    );
}

fn record_next_direction_change(game_state: &mut GameState, direction: Direction) {
    if game_state.direction_queue.len() == MAX_MOVEMENT_QUEUE {
        game_state.direction_queue.clear();
    }
    game_state.direction_queue.push(direction);
}

fn is_in_snake(x: i32, y: i32, snake: &Snake) -> bool {
    for i in 0..snake.body.len() {
        if snake.body[i].x == x && snake.body[i].y == y {
            return true;
        }
    }
    false
}

fn draw_food(game_state: &mut GameState, d: &mut RaylibDrawHandle) {
    if game_state.food.new_food {
        let mut rng = rand::thread_rng();
        let mut x = (rng.gen_range(SNAKE_SIZE..WINDOW_WIDTH - SNAKE_SIZE) / GRID_SQUARE_SIZE)
            * GRID_SQUARE_SIZE;
        let mut y = (rng.gen_range(SNAKE_SIZE..WINDOW_HEIGHT - SNAKE_SIZE) / GRID_SQUARE_SIZE)
            * GRID_SQUARE_SIZE;
        while is_in_snake(x, y, &game_state.snake) {
            x = (rng.gen_range(SNAKE_SIZE..WINDOW_WIDTH - SNAKE_SIZE) / GRID_SQUARE_SIZE)
                * GRID_SQUARE_SIZE;
            y = (rng.gen_range(SNAKE_SIZE..WINDOW_HEIGHT - SNAKE_SIZE) / GRID_SQUARE_SIZE)
                * GRID_SQUARE_SIZE;
        }
        game_state.food.position.x = x;
        game_state.food.position.y = y;
        game_state.food.new_food = false;
    } else {
        if is_in_snake(
            game_state.food.position.x,
            game_state.food.position.y,
            &game_state.snake,
        ) {
            game_state.food.eaten = true;
            game_state.food.new_food = true;
            game_state.snake.speed += SNAKE_SPEED_INCREASE;
        }
    }
    d.draw_circle(
        game_state.food.position.x + FOOD_RADIUS,
        game_state.food.position.y + FOOD_RADIUS,
        FOOD_RADIUS as f32,
        Color::BLACK,
    );
}

fn move_snake(game_state: &mut GameState) {
    if game_state.time_since_last_move > (1. / game_state.snake.speed) {
        game_state.time_since_last_move = 0.0;
        if let Some(next_direction) = game_state.direction_queue.pop() {
            if matches!(next_direction, Direction::Up) {
                if !matches!(game_state.snake.direction, Direction::Down) {
                    game_state.snake.direction = Direction::Up;
                }
            }
            if matches!(next_direction, Direction::Down) {
                if !matches!(game_state.snake.direction, Direction::Up) {
                    game_state.snake.direction = Direction::Down;
                }
            }
            if matches!(next_direction, Direction::Left) {
                if !matches!(game_state.snake.direction, Direction::Right) {
                    game_state.snake.direction = Direction::Left;
                }
            }
            if matches!(next_direction, Direction::Right) {
                if !matches!(game_state.snake.direction, Direction::Left) {
                    game_state.snake.direction = Direction::Right;
                }
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

        if !game_state.food.eaten {
            game_state.snake.body.remove(0);
        } else {
            game_state.food.eaten = false
        }
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
    false
}

fn draw_score(score: usize, d: &mut RaylibDrawHandle) {
    d.draw_text(&format!("{}", score), 10, 10, 12, Color::BLACK);
}
