use crate::Ball;
use crate::BLOCK_SIZE;
use crate::Block;
use crate::block::BlockType;
use crate::player::Player;
use macroquad::prelude::*;

enum GameState {
    START,
    GAME,
    WIN,
    DEAD,
}

pub struct RustBreakout{
    state: GameState,
    player: Player,
    balls: Vec<Ball>,
    blocks: Vec<Block>,
    font: Font
}

impl RustBreakout{

    pub fn new(font: Font) -> Self{
        let player = Player::new();
        let balls = Vec::new();
        let blocks = Vec::new();
        RustBreakout { player, balls, blocks , state: GameState::START, font}
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
            *vel = vel.normalize();
        }
        true
    }

    /**
     * Rearranges the blocks
     */
    fn setup_blocks(&mut self, width: u16, height: u16) {
        let padding = 5f32;
        let total_block_size = BLOCK_SIZE + vec2(padding, padding);
        let board_start_pos = vec2(
            (screen_width() - (total_block_size.x * width as f32)) * 0.5f32,
            50f32,
        );
        for i in 0..width * height {
            let block_x = (i % width) as f32 * (total_block_size.x);
            let block_y = (i / width) as f32 * (total_block_size.y);
            let rng = rand::gen_range(0, 3);
            let block_type = match &rng {
                0 => BlockType::BallSpawner,
                _ => BlockType::Normal
            };
            let mut block = Block::new(board_start_pos + vec2(block_x, block_y), block_type);

            let row_count = (i / width) as f32;
            if row_count == 0f32 {
                block.lives = 3;
            } else if row_count <= 2f32 {
                block.lives = 2;
            }
            self.blocks.push(block);
        }
    }

    /**
     * Resets the game
     */
    fn reset_game(&mut self) {
        self.balls = Vec::new();
        self.setup_blocks(6, 6);
        self.player = Player::new();
        self.state = GameState::START;
    }

    fn eliminate_balls(&mut self) {
        let balls_before_remove = self.balls.len();
        self.balls.retain(|ball| ball.rect.y < screen_height() - 50f32);
        let balls_after_remove = self.balls.len();
        if balls_before_remove != balls_after_remove && balls_after_remove <= 0 && self.player.lives > 0 {
            self.player.lives -= 1;
        }
        if self.player.lives < 1 {
            self.state = GameState::DEAD;
        }
    }

    fn draw_lives(&mut self) {
        let text = format!("Lives: {}", self.player.lives);
        draw_text_ex(
            &text,
            10f32,
            50f32,
            TextParams {
                font: self.font,
                font_size: 30u16,
                color: WHITE,
                ..Default::default()
            },
        );
    }

    pub fn handle_frame(&mut self){
        match self.state {
            GameState::START => {
                self.player.draw();
                self.draw_lives();
                for ball in self.balls.iter_mut() {
                    ball.draw();
                }
                for block in self.blocks.iter_mut() {
                    block.draw();
                }
                let text = "Press SPACE to start!";
                let text_size = measure_text(text, Some(self.font), 30u16, 1.0);
                draw_text_ex(
                    text,
                    screen_width() * 0.5f32 - text_size.width * 0.5f32,
                    screen_height() * 0.5f32 - text_size.height * 0.5f32,
                    TextParams {
                        font: self.font,
                        font_size: 30u16,
                        color: WHITE,
                        ..Default::default()
                    },
                );

                if is_key_pressed(KeyCode::Space) {
                    self.setup_blocks(6, 6);
                    self.state = GameState::GAME;
                }
            },
            GameState::GAME => {
                if is_key_pressed(KeyCode::Space) {
                    let ball = Ball::new(vec2(
                        self.player.rect.x + self.player.rect.w * 0.5f32,
                        self.player.rect.y - 100f32,
                    ));
                    self.balls.push(ball);
                    println!("Space pressed");
                } else if is_key_pressed(KeyCode::R) {
                    self.setup_blocks(6, 6);
                }
                self.player.update(get_frame_time());
                for block in self.blocks.iter() {
                    block.draw();
                }
                let mut balls_to_spawn: Vec<Ball> = Vec::new();
                for ball in self.balls.iter_mut() {
                    ball.update(get_frame_time());
                    ball.draw();
                    self.player.resolve_collision(ball);
                    for block in self.blocks.iter_mut() {
                        if RustBreakout::resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect) {
                            if block.lives > 0 {
                                block.lives -= 1;
                            }
                            if block.lives <= 0{
                                if let BlockType::BallSpawner = block.block_type {
                                    balls_to_spawn.push(
                                        Ball::new(Vec2{x: ball.rect.center().x, y: ball.rect.center().y + (ball.rect.h / 2f32)})
                                    )
                                }
                            }
                        }
                    }
                }
                for ball in balls_to_spawn {
                    self.balls.push(ball);
                }
                self.blocks.retain(|block| block.lives > 0);
                self.eliminate_balls();
                self.player.draw();
                self.draw_lives();
                // Check for win
                if self.blocks.is_empty() {
                    self.state = GameState::WIN;
                }
            },
            GameState::WIN => {
                if is_key_pressed(KeyCode::Space) {
                    self.reset_game();
                    self.state = GameState::GAME;
                }
                self.player.draw();
                self.draw_lives();
                for ball in self.balls.iter_mut() {
                    ball.draw();
                }
                for block in self.blocks.iter_mut() {
                    block.draw();
                }
                let text = "You Won!\nPress SPACE to start again.";
                let font_size = measure_text(text, Some(self.font), 30u16, 1.0);
                draw_text_ex(
                    text,
                    screen_width() * 0.5f32 - font_size.width * 0.5f32,
                    screen_height() * 0.5f32,
                    TextParams {
                        font: self.font,
                        font_size: 30u16,
                        color: WHITE,
                        ..Default::default()
                    },
                );
            },
            GameState::DEAD => {
                if is_key_pressed(KeyCode::Space) {
                    self.reset_game();
                    self.state = GameState::GAME;
                }
                self.player.draw();
                self.draw_lives();
                for ball in self.balls.iter_mut() {
                    ball.draw();
                }
                for block in self.blocks.iter_mut() {
                    block.draw();
                }
                let text = "You're dead.\nPress SPACE to start again.";
                let font_size = measure_text(text, Some(self.font), 30u16, 1.0);
                draw_text_ex(
                    text,
                    screen_width() * 0.5f32 - font_size.width * 0.5f32,
                    screen_height() * 0.5f32,
                    TextParams {
                        font: self.font,
                        font_size: 30u16,
                        color: WHITE,
                        ..Default::default()
                    },
                );
            },
        }
    }
}
