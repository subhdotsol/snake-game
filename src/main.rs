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

/// Pick a random cell not occupied by snake, foods, or obstacles
fn free_cell(snake: &[Point], foods: &[Point], obstacles: &[Point]) -> Point {
    loop {
        let p = Point {
            x: rand::gen_range(0, GRID_SIZE),
            y: rand::gen_range(0, GRID_SIZE),
        };
        if !snake.contains(&p) && !foods.contains(&p) && !obstacles.contains(&p) {
            return p;
        }
    }
}

/// Spawn 1-3 food items at random free cells
fn spawn_foods(count: i32, snake: &[Point], foods: &mut Vec<Point>, obstacles: &[Point]) {
    for _ in 0..count {
        let p = free_cell(snake, foods, obstacles);
        foods.push(p);
    }
}

/// Spawn a cluster of 1-5 connected obstacle blocks
fn spawn_obstacle_cluster(snake: &[Point], foods: &[Point], obstacles: &mut Vec<Point>) {
    let cluster_len = rand::gen_range(1_i32, 6); // 1 to 5 blocks
    let start = free_cell(snake, foods, obstacles);
    obstacles.push(start);

    let mut cur = start;
    let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for _ in 1..cluster_len {
        // try a random adjacent cell up to 10 times
        let mut placed = false;
        for _ in 0..10 {
            let d = dirs[rand::gen_range(0_usize, 4)];
            let next = Point { x: cur.x + d.0, y: cur.y + d.1 };
            if next.x >= 0
                && next.x < GRID_SIZE
                && next.y >= 0
                && next.y < GRID_SIZE
                && !snake.contains(&next)
                && !foods.contains(&next)
                && !obstacles.contains(&next)
            {
                obstacles.push(next);
                cur = next;
                placed = true;
                break;
            }
        }
        if !placed {
            break; // couldn't extend, stop cluster here
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
    let mut foods: Vec<Point> = Vec::new();
    let mut obstacles: Vec<Point> = Vec::new();
    let mut last_move_time = get_time();
    let mut move_delay = 0.15_f64;
    let mut food_count = 0_u32;
    let mut game_over = false;

    // Start with 1-3 fruits on the board
    spawn_foods(rand::gen_range(1_i32, 4), &snake, &mut foods, &obstacles);

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

                    // Check if head landed on any food
                    if let Some(idx) = foods.iter().position(|f| *f == new_head) {
                        foods.remove(idx);
                        food_count += 1;

                        // Every 2 fruits eaten → change speed AND spawn obstacle cluster
                        if food_count % 2 == 0 {
                            // Speed: randomly +5..+15% faster OR -5..-15% slower (truly random)
                            let pct = rand::gen_range(5_i32, 16) as f64 / 100.0;
                            if rand::gen_range(0_i32, 2) == 0 {
                                move_delay -= move_delay * pct; // faster (lower delay)
                            } else {
                                move_delay += move_delay * pct; // slower (higher delay)
                            }
                            move_delay = move_delay.clamp(0.05, 0.30);

                            // Spawn obstacle cluster (1-5 blocks)
                            spawn_obstacle_cluster(&snake, &foods, &mut obstacles);
                        }

                        // Respawn 1-3 fruits to replace eaten one
                        let new_fruit_count = rand::gen_range(1_i32, 4);
                        spawn_foods(new_fruit_count, &snake, &mut foods, &obstacles);
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
                foods.clear();
                obstacles.clear();
                move_delay = 0.15;
                food_count = 0;
                game_over = false;
                spawn_foods(rand::gen_range(1_i32, 4), &snake, &mut foods, &obstacles);
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

        // Draw all food items (red)
        for food in &foods {
            draw_rectangle(
                OFFSET + food.x as f32 * CELL_SIZE,
                OFFSET + food.y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                RED,
            );
        }

        next_frame().await;
    }
}
