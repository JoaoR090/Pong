use ggez::{
    graphics::Rect,
};

pub struct Raquete {
    pub rect: Rect,
    pub velocidade: f32,
}

impl Raquete {
    pub fn new(x: f32, y: f32, w: f32, h: f32, velocidade: f32) -> Self{
        Raquete{
            rect: Rect::new(x, y, w, h),
            velocidade: velocidade,
        }
    }
}
