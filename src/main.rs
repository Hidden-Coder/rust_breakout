mod ball;
mod block;
mod player;
mod game;

use ball::Ball;
use block::{Block, BLOCK_SIZE};
use macroquad::prelude::*;

fn window_conf() -> Conf {
        Conf{
            window_resizable: false,
            window_width: 1000,
            window_height: 600,
            ..Default::default()
        }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font = load_ttf_font("src/res/DragonFruitDays.ttf").await.unwrap();
    next_frame().await;
    let mut game = game::RustBreakout::new(font);

    loop {
        game.handle_frame();
        next_frame().await;
    }
}
