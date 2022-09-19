use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);
const PLAYER_SPEED: f32 = 700f32;
const BLOCK_SIZE: Vec2 = Vec2::from_array([100f32, 40f32]);
const BALL_SIZE: Vec2 = Vec2::from_array([40f32, 40f32]);
const BALL_SPEED: f32 = 500f32;

struct Player{
    rect: Rect,
}

impl Player{
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y
            )
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut x_move = 0f32;
        if is_key_down(KeyCode::D){x_move += 1f32}
        if is_key_down(KeyCode::A) {x_move += -1f32}

        self.rect.x += x_move * dt * PLAYER_SPEED;

        if self.rect.x < 0f32 {self.rect.x = 0f32}
        if self.rect.x > screen_width() - PLAYER_SIZE.x {self.rect.x = screen_width() - PLAYER_SIZE.x}
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}


struct Block {
    rect: Rect,
    lives: u8,
}

impl Block {
    pub fn new(pos: Vec2) -> Self{
        Block{
            rect: Rect { x: pos.x, y: pos.y, w: BLOCK_SIZE.x, h: BLOCK_SIZE.y },
            lives: 1u8,
        }
    }

    pub fn draw(&self){
        let color = match self.lives {
            3 => DARKBROWN,
            2 => RED,
            _ => BLUE
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}


struct Ball{
    rect: Rect,
    vel: Vec2,
}

impl Ball{
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect { x: pos.x, y: pos.y, w: BALL_SIZE.x, h: BALL_SIZE.y },
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }

    pub fn update(&mut self, dt: f32){
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;

        if self.rect.x < 0f32 || self.rect.x > screen_width() - self.rect.w {self.vel.x *= -1f32}
        if self.rect.y < 0f32 {self.vel.y *= -1f32}
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, YELLOW);
    }
}

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
        if distance_signum.y > 0f32 {vel.y = -vel.y.abs();}
        else {vel.y = vel.y.abs();}
    }
    else {
        a.x -= distance_signum.x * intersection.w;
        if distance_signum.x > 0f32 {vel.x = -vel.x.abs();}
        else {vel.x = vel.x.abs();}
    }
    true
}

#[macroquad::main("breakout")]
async fn main() {
    let mut player = Player::new();
    let mut blocks : Vec<Block>= Vec::new();
    let mut balls: Vec<Ball> = Vec::new();
    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);

    let (width, height) = (6,6);
    let board_start_pos = vec2((screen_width() - (total_block_size.x * width as f32)) * 0.5f32, 50f32);
    for i in 0..width * height {
        let block_x = (i % width) as f32 * (total_block_size.x);
        let block_y = (i / width) as f32 * (total_block_size.y);
        let mut block = Block::new(board_start_pos + vec2(block_x, block_y));
        
        let row_count = (i / width) as f32;
        if row_count == 0f32 {block.lives = 3;}
        else if row_count <= 2f32 {block.lives = 2;}
        blocks.push(block);
    }

    loop{
        if is_key_pressed(KeyCode::Space) {
            let ball = Ball::new(vec2(player.rect.x + player.rect.w * 0.5f32, player.rect.y - 100f32));
            balls.push(ball);
            println!("Space pressed");
        }

        player.update(get_frame_time());

        for block in blocks.iter() {block.draw();}        
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
