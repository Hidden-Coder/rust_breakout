use macroquad::prelude::*;

use crate::ball::Ball;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);
const PLAYER_SPEED: f32 = 700f32;

pub struct Player {
    pub rect: Rect,
    pub lives: u8,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
            lives: 3u8,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut x_move = 0f32;
        if is_key_down(KeyCode::D) {
            x_move += 1f32
        }
        if is_key_down(KeyCode::A) {
            x_move += -1f32
        }

        self.rect.x += x_move * dt * PLAYER_SPEED;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32
        }
        if self.rect.x > screen_width() - PLAYER_SIZE.x {
            self.rect.x = screen_width() - PLAYER_SIZE.x
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }


    pub fn resolve_collision(&mut self, ball: &mut Ball) -> bool {
        let intersection = match self.rect.intersect(ball.rect){
            Some(intersection) => intersection,
            None => return false
        };
        let self_center = self.rect.center();
        let ball_center = ball.rect.center();
        let distance_vec = self_center - ball_center;
        let distance_signum  = distance_vec.signum();
        if intersection.w > intersection.h {
            ball.rect.y -= distance_signum.y * intersection.h;
            if distance_signum.y > 0f32{
                ball.vel.y = -ball.vel.y.abs();
            } else {
                ball.vel.y = ball.vel.y.abs();
            }
            ball.vel.x -= distance_vec.normalize().x * 0.5f32;
            ball.vel = ball.vel.normalize();
        } else {
            ball.rect.x -= distance_signum.x * intersection.w;
            if distance_signum.x > 0f32 {
                ball.vel.x = -ball.vel.x.abs();
            } else {
                ball.vel.x = ball.vel.x.abs();
            }
        }
        return true;
    }
}
