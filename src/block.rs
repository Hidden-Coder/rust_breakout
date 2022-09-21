use macroquad::prelude::*;

pub const BLOCK_SIZE: Vec2 = Vec2::from_array([100f32, 40f32]);

pub struct Block {
    pub rect: Rect,
    pub lives: u8,
}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        Block {
            rect: Rect {
                x: pos.x,
                y: pos.y,
                w: BLOCK_SIZE.x,
                h: BLOCK_SIZE.y,
            },
            lives: 1u8,
        }
    }

    pub fn draw(&self) {
        let color = match self.lives {
            3 => DARKBROWN,
            2 => RED,
            _ => BLUE,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}
