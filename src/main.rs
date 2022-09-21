mod ball;
mod block;
mod player;

use ball::Ball;
use block::{Block, BLOCK_SIZE};
use macroquad::prelude::*;
use player::Player;

enum GameState {
    START,
    GAME,
    WIN,
    DEAD,
}

/**
 * Function for resolving collisions of all sorts
 */
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

/**
 * Rearranges the blocks
 */
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

/**
 * Resets the game
 */
fn reset_game(
    blocks: &mut Vec<Block>,
    game_state: &mut GameState,
    balls: &mut Vec<Ball>,
    player: &mut Player,
) {
    setup_blocks(blocks, 6, 6);
    *balls = Vec::new();
    *player = Player::new();
    *game_state = GameState::START;
}

fn eliminate_balls(balls: &mut Vec<Ball>, player: &mut Player, game_state: &mut GameState) {
    let balls_before_remove = balls.len();
    balls.retain(|ball| ball.rect.y < screen_height() - 50f32);
    let balls_after_remove = balls.len();
    if balls_before_remove != balls_after_remove && balls_after_remove <= 0 && player.lives > 0 {
        player.lives -= 1;
    }
    if player.lives < 1 {
        *game_state = GameState::DEAD;
    }
}

fn draw_lives(player: &Player, font: &Font) {
    let text = format!("Lives: {}", player.lives);
    draw_text_ex(
        &text,
        10f32,
        50f32,
        TextParams {
            font: *font,
            font_size: 30u16,
            color: WHITE,
            ..Default::default()
        },
    );
}

#[macroquad::main("breakout")]
async fn main() {
    let mut player = Player::new();
    let mut blocks: Vec<Block> = Vec::new();
    let mut balls: Vec<Ball> = Vec::new();
    let mut game_state = GameState::START;
    let font = load_ttf_font("src/res/DragonFruitDays.ttf").await.unwrap();
    setup_blocks(&mut blocks, 6, 6);

    loop {
        match game_state {
            GameState::START => {
                player.draw();
                draw_lives(&player, &font);
                for ball in balls.iter_mut() {
                    ball.draw();
                }
                for block in blocks.iter_mut() {
                    block.draw();
                }
                let text = "Press SPACE to start!";
                let text_size = measure_text(text, Some(font), 30u16, 1.0);
                draw_text_ex(
                    text,
                    screen_width() * 0.5f32 - text_size.width * 0.5f32,
                    screen_height() * 0.5f32 - text_size.height * 0.5f32,
                    TextParams {
                        font,
                        font_size: 30u16,
                        color: WHITE,
                        ..Default::default()
                    },
                );

                if is_key_pressed(KeyCode::Space) {
                    setup_blocks(&mut blocks, 6, 6);
                    game_state = GameState::GAME;
                }
            }
            GameState::GAME => {
                if is_key_pressed(KeyCode::Space) {
                    let ball = Ball::new(vec2(
                        player.rect.x + player.rect.w * 0.5f32,
                        player.rect.y - 100f32,
                    ));
                    balls.push(ball);
                    println!("Space pressed");
                } else if is_key_pressed(KeyCode::R) {
                    setup_blocks(&mut blocks, 6, 6);
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
                            if block.lives > 0 {
                                block.lives -= 1;
                            }
                        }
                    }
                }
                blocks.retain(|block| block.lives > 0);
                eliminate_balls(&mut balls, &mut player, &mut game_state);
                player.draw();
                draw_lives(&player, &font);
                // Check for win
                if blocks.is_empty() {
                    game_state = GameState::WIN;
                }
            }
            GameState::WIN => {
                if is_key_pressed(KeyCode::Space) {
                    reset_game(&mut blocks, &mut game_state, &mut balls, &mut player);
                    game_state = GameState::GAME;
                }
                player.draw();
                draw_lives(&player, &font);
                for ball in balls.iter_mut() {
                    ball.draw();
                }
                for block in blocks.iter_mut() {
                    block.draw();
                }
                let text = "You Won!\nPress SPACE to start again.";
                let font_size = measure_text(text, Some(font), 30u16, 1.0);
                draw_text_ex(
                    text,
                    screen_width() * 0.5f32 - font_size.width * 0.5f32,
                    screen_height() * 0.5f32,
                    TextParams {
                        font,
                        font_size: 30u16,
                        color: WHITE,
                        ..Default::default()
                    },
                );
            }
            GameState::DEAD => {
                if is_key_pressed(KeyCode::Space) {
                    reset_game(&mut blocks, &mut game_state, &mut balls, &mut player);
                    game_state = GameState::GAME;
                }
                player.draw();
                draw_lives(&player, &font);
                for ball in balls.iter_mut() {
                    ball.draw();
                }
                for block in blocks.iter_mut() {
                    block.draw();
                }
                let text = "You're dead.\nPress SPACE to start again.";
                let font_size = measure_text(text, Some(font), 30u16, 1.0);
                draw_text_ex(
                    text,
                    screen_width() * 0.5f32 - font_size.width * 0.5f32,
                    screen_height() * 0.5f32,
                    TextParams {
                        font,
                        font_size: 30u16,
                        color: WHITE,
                        ..Default::default()
                    },
                );
            }
        }
        next_frame().await;
    }
}
