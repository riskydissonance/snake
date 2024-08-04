use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 450;
const SNAKE_SIZE: i32 = 10;

enum Direction {
    Up,
    Down,
    Left,
    Right,
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

    let mut game_over = false;
    rl.set_target_fps(120);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if game_over || check_game_over(&snake) {
            game_over = true;
            d.draw_text("Game Over", 320, 225, 20, Color::RED);
            continue;
        }

        d.clear_background(Color::DARKGREEN);
        d.draw_text(
            "This is snake, but it's not done yet.",
            190,
            200,
            20,
            Color::LIGHTGRAY,
        );

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
        draw_snake(&snake, &mut d);
    }
}

fn draw_snake(snake: &Snake, d: &mut RaylibDrawHandle) {
    d.draw_rectangle(snake.x, snake.y, SNAKE_SIZE, SNAKE_SIZE, Color::BLACK);
    for i in 0..snake.length {
        d.draw_rectangle(
            snake.x,
            snake.y - i * SNAKE_SIZE + 1,
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
