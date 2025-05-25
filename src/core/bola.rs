use ggez::{
    graphics::Rect,
};

pub struct Bola {
    pub rect: Rect,
    pub x_vel: f32,
    pub y_vel: f32,
}

impl Bola {
    pub fn new(w: f32, h: f32, x_vel: f32, y_vel: f32) -> Self {
        Bola {
            rect: Rect::new(390.0, 290.0, w, h),
            x_vel: x_vel,
            y_vel: y_vel,
        }
    }    

    pub fn get_rect(&self) -> Rect{
        self.rect
    }

     pub fn mover(&mut self) {
        self.mover_x(self.x_vel);
        self.mover_y(self.y_vel);
    }

    pub fn mover_x(&mut self, dx: f32) {
        self.rect.x += dx;
    }

    pub fn mover_y(&mut self, dy: f32) {
        self.rect.y += dy;
    }
}