use macroquad::{prelude::*, rand};

const GRID_SIZE: i32 = 20;
const CELL_SIZE: f32 = 20.0;

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn random_food(snake: &Vec<Point>) -> Point {
    loop {
        let p = Point {
            x: rand::gen_range(0, GRID_SIZE),
            y: rand::gen_range(0, GRID_SIZE),
        };
        if !snake.contains(&p) {
            return p;
        }
    }
}

#[macroquad::main("Snake Game")]
async fn main() {
    let mut snake = vec![
        Point { x: 5, y: 5 },
        Point { x: 4, y: 5 },
        Point { x: 3, y: 5 },
    ];
    let mut dir = Direction::Right;
    let mut food = random_food(&snake);
    let mut last_move_time = get_time();
    let move_delay = 0.15;
    let mut game_over = false;

    loop {
        clear_background(BLACK);

        if !game_over {
            if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) && dir != Direction::Down {
                dir = Direction::Up;
            }
            if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) && dir != Direction::Up {
                dir = Direction::Down;
            }
            if is_key_pressed(KeyCode::Left)
                || is_key_pressed(KeyCode::A) && dir != Direction::Right
            {
                dir = Direction::Left;
            }
            if is_key_pressed(KeyCode::Right)
                || is_key_pressed(KeyCode::D) && dir != Direction::Left
            {
                dir = Direction::Right;
            }

            if get_time() - last_move_time > move_delay {
                last_move_time = get_time();
                let mut new_head = snake[0];
                match dir {
                    Direction::Up => new_head.y -= 1,
                    Direction::Down => new_head.y += 1,
                    Direction::Left => new_head.x -= 1,
                    Direction::Right => new_head.x += 1,
                }

                if new_head.x < 0
                    || new_head.x >= GRID_SIZE
                    || new_head.y < 0
                    || new_head.y >= GRID_SIZE
                    || snake.contains(&new_head)
                {
                    game_over = true;
                } else {
                    snake.insert(0, new_head);
                    if new_head == food {
                        food = random_food(&snake);
                    } else {
                        snake.pop();
                    }
                }
            }
        } else {
            draw_text("GAME OVER! Press R to Restart", 40.0, 200.0, 30.0, WHITE);
            if is_key_pressed(KeyCode::R) {
                snake = vec![
                    Point { x: 5, y: 5 },
                    Point { x: 4, y: 5 },
                    Point { x: 3, y: 5 },
                ];
                dir = Direction::Right;
                food = random_food(&snake);
                game_over = false;
            }
        }

        for (i, segment) in snake.iter().enumerate() {
            let color = if i == 0 { YELLOW } else { GREEN };
            draw_rectangle(
                segment.x as f32 * CELL_SIZE,
                segment.y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                color,
            );
        }
        draw_rectangle(
            food.x as f32 * CELL_SIZE,
            food.y as f32 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
            RED,
        );

        next_frame().await;
    }
}
