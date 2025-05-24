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

    pub fn top(&self) -> f32{
        self.rect.top()
    }

    pub fn bottom(&self) -> f32{
        self.rect.bottom()
    }

    pub fn left(&self) -> f32{
        self.rect.left()
    }

    pub fn right(&self) -> f32{
        self.rect.right()
    }

    pub fn rect(&self) -> Rect{
        self.rect
    }
}