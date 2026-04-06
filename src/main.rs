use macroquad::{prelude::*, rand};

const GRID_SIZE: i32 = 38;
const CELL_SIZE: f32 = 20.0;
const OFFSET: f32 = 20.0;
const BORDER_THICKNESS: f32 = 3.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake Game".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

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

/// Pick a random free cell (not on snake, food, or obstacles)
fn random_food(snake: &[Point], food: &Point, obstacles: &[Point]) -> Point {
    loop {
        let p = Point {
            x: rand::gen_range(0, GRID_SIZE),
            y: rand::gen_range(0, GRID_SIZE),
        };
        if !snake.contains(&p) && p != *food && !obstacles.contains(&p) {
            return p;
        }
    }
}

fn initial_food(snake: &[Point], obstacles: &[Point]) -> Point {
    loop {
        let p = Point {
            x: rand::gen_range(0, GRID_SIZE),
            y: rand::gen_range(0, GRID_SIZE),
        };
        if !snake.contains(&p) && !obstacles.contains(&p) {
            return p;
        }
    }
}

/// Get a random shape pattern (offsets from anchor point)
fn random_shape() -> Vec<(i32, i32)> {
    match rand::gen_range(0_i32, 8) {
        // horizontal line (3-5 blocks)
        0 => { let len = rand::gen_range(3_i32, 6); (0..len).map(|i| (i, 0)).collect() }
        // vertical line (3-5 blocks)
        1 => { let len = rand::gen_range(3_i32, 6); (0..len).map(|i| (0, i)).collect() }
        // L-shape
        2 => vec![(0,0),(1,0),(2,0),(2,1),(2,2)],
        // reverse L
        3 => vec![(0,0),(0,1),(0,2),(1,0),(2,0)],
        // T-shape
        4 => vec![(0,0),(1,0),(2,0),(1,1),(1,2)],
        // plus/cross
        5 => vec![(1,0),(0,1),(1,1),(2,1),(1,2)],
        // small square 2x2
        6 => vec![(0,0),(1,0),(0,1),(1,1)],
        // Z-shape
        _ => vec![(0,0),(1,0),(1,1),(2,1)],
    }
}

/// Spawn a random shaped obstacle at a random location
fn spawn_obstacle(snake: &[Point], food: &Point, obstacles: &mut Vec<Point>) {
    // try up to 50 times to find a valid placement
    for _ in 0..50 {
        let shape = random_shape();
        let anchor_x = rand::gen_range(1_i32, GRID_SIZE - 6);
        let anchor_y = rand::gen_range(1_i32, GRID_SIZE - 6);

        let cells: Vec<Point> = shape.iter()
            .map(|(dx, dy)| Point { x: anchor_x + dx, y: anchor_y + dy })
            .collect();

        // check all cells are valid
        let valid = cells.iter().all(|c| {
            c.x >= 0 && c.x < GRID_SIZE
            && c.y >= 0 && c.y < GRID_SIZE
            && !snake.contains(c)
            && *c != *food
            && !obstacles.contains(c)
        });

        if valid {
            obstacles.extend(cells);
            return;
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut snake = vec![
        Point { x: 5, y: 5 },
        Point { x: 4, y: 5 },
        Point { x: 3, y: 5 },
    ];
    let mut dir = Direction::Right;
    let mut obstacles: Vec<Point> = Vec::new();
    let mut food = initial_food(&snake, &obstacles);
    let mut last_move_time = get_time();
    let mut move_delay = 0.15_f64;
    let mut food_count = 0_u32;
    let mut game_over = false;



    loop {
        clear_background(Color::from_rgba(15, 15, 15, 255));

        if !game_over {
            // Input
            if is_key_pressed(KeyCode::Up) && dir != Direction::Down { dir = Direction::Up; }
            if is_key_pressed(KeyCode::Down) && dir != Direction::Up { dir = Direction::Down; }
            if is_key_pressed(KeyCode::Left) && dir != Direction::Right { dir = Direction::Left; }
            if is_key_pressed(KeyCode::Right) && dir != Direction::Left { dir = Direction::Right; }
            if is_key_pressed(KeyCode::W) && dir != Direction::Down { dir = Direction::Up; }
            if is_key_pressed(KeyCode::S) && dir != Direction::Up { dir = Direction::Down; }
            if is_key_pressed(KeyCode::A) && dir != Direction::Right { dir = Direction::Left; }
            if is_key_pressed(KeyCode::D) && dir != Direction::Left { dir = Direction::Right; }

            if get_time() - last_move_time > move_delay {
                last_move_time = get_time();
                let mut new_head = snake[0];
                match dir {
                    Direction::Up    => new_head.y -= 1,
                    Direction::Down  => new_head.y += 1,
                    Direction::Left  => new_head.x -= 1,
                    Direction::Right => new_head.x += 1,
                }

                // Collision: wall, self, or obstacle
                if new_head.x < 0
                    || new_head.x >= GRID_SIZE
                    || new_head.y < 0
                    || new_head.y >= GRID_SIZE
                    || snake.contains(&new_head)
                    || obstacles.contains(&new_head)
                {
                    game_over = true;
                } else {
                    snake.insert(0, new_head);

                    // Check if head landed on the food
                    if new_head == food {
                        food_count += 1;

                        // Every 2 fruits → change speed + spawn obstacle shape
                        if food_count % 2 == 0 {
                            let pct = rand::gen_range(5_i32, 16) as f64 / 100.0;
                            if rand::gen_range(0_i32, 2) == 0 {
                                move_delay -= move_delay * pct;
                            } else {
                                move_delay += move_delay * pct;
                            }
                            move_delay = move_delay.clamp(0.05, 0.30);

                            // Spawn a random shaped obstacle
                            spawn_obstacle(&snake, &food, &mut obstacles);
                        }

                        // Spawn new single fruit
                        food = random_food(&snake, &food, &obstacles);
                    } else {
                        snake.pop(); // didn't eat, remove tail
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
                obstacles.clear();
                food = initial_food(&snake, &obstacles);
                move_delay = 0.15;
                food_count = 0;
                game_over = false;
            }
        }

        // Draw border
        let grid_px = GRID_SIZE as f32 * CELL_SIZE;
        draw_rectangle_lines(
            OFFSET - BORDER_THICKNESS,
            OFFSET - BORDER_THICKNESS,
            grid_px + BORDER_THICKNESS * 2.0,
            grid_px + BORDER_THICKNESS * 2.0,
            BORDER_THICKNESS * 2.0,
            WHITE,
        );

        // Draw snake
        for (i, seg) in snake.iter().enumerate() {
            let color = if i == 0 { YELLOW } else { GREEN };
            draw_rectangle(
                OFFSET + seg.x as f32 * CELL_SIZE,
                OFFSET + seg.y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                color,
            );
        }

        // Draw obstacles (orange clusters)
        for obs in &obstacles {
            draw_rectangle(
                OFFSET + obs.x as f32 * CELL_SIZE,
                OFFSET + obs.y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                ORANGE,
            );
        }

        // Draw food (single red block)
        draw_rectangle(
            OFFSET + food.x as f32 * CELL_SIZE,
            OFFSET + food.y as f32 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
            RED,
        );

        next_frame().await;
    }
}
