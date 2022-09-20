mod ball;
mod player;
mod block;

use block::{Block, BLOCK_SIZE};
use ball::Ball;
use player::Player;
use macroquad::prelude::*;


fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };
    let a_center = a.center();
    let b_center = b.center();
    let distance_vec = b_center - a_center;
    let distance_signum = distance_vec.signum();
    if intersection.w > intersection.h {
        a.y -= distance_signum.y * intersection.h;
        if distance_signum.y > 0f32 {
            vel.y = -vel.y.abs();
        } else {
            vel.y = vel.y.abs();
        }
    } else {
        a.x -= distance_signum.x * intersection.w;
        if distance_signum.x > 0f32 {
            vel.x = -vel.x.abs();
        } else {
            vel.x = vel.x.abs();
        }
    }
    true
}



fn setup_blocks(blocks_array: &mut Vec<Block>, width: i32, height: i32) {
    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2(
        (screen_width() - (total_block_size.x * width as f32)) * 0.5f32,
        50f32,
    );
    for i in 0..width * height {
        let block_x = (i % width) as f32 * (total_block_size.x);
        let block_y = (i / width) as f32 * (total_block_size.y);
        let mut block = Block::new(board_start_pos + vec2(block_x, block_y));

        let row_count = (i / width) as f32;
        if row_count == 0f32 {
            block.lives = 3;
        } else if row_count <= 2f32 {
            block.lives = 2;
        }
        blocks_array.push(block);
    }
}

#[macroquad::main("breakout")]
async fn main() {
    let mut player = Player::new();
    let mut blocks: Vec<Block> = Vec::new();
    let mut balls: Vec<Ball> = Vec::new();

    setup_blocks(&mut blocks, 6, 6);

    loop {
        if is_key_pressed(KeyCode::Space) {
            let ball = Ball::new(vec2(
                player.rect.x + player.rect.w * 0.5f32,
                player.rect.y - 100f32,
            ));
            balls.push(ball);
            println!("Space pressed");
        }

        player.update(get_frame_time());

        for block in blocks.iter() {
            block.draw();
        }
        for ball in balls.iter_mut() {
            ball.update(get_frame_time());
            ball.draw();
            resolve_collision(&mut ball.rect, &mut ball.vel, &player.rect);
            for block in blocks.iter_mut() {
                if resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect) {
                    block.lives -= 1;
                }
            }
        }

        blocks.retain(|block| block.lives > 0);
        player.draw();
        next_frame().await;
    }
}
