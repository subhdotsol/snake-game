use macroquad::prelude::*;

#[macroquad::main("Snake Game")]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await;
    }
}
