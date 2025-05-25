use ggez::{
    graphics::Rect,
};

pub enum LadoRaquete{
    E,
    D,
}

pub struct Raquete {
    pub rect: Rect,
    pub velocidade: f32,
    pub lado: LadoRaquete,
}

impl Raquete {
    pub fn new(x: f32, y: f32, w: f32, h: f32, velocidade: f32, lado: LadoRaquete) -> Self{
        Raquete{
            rect: Rect::new(x, y, w, h),
            velocidade: velocidade,
            lado: lado,
        }
    }

    pub fn mover_para_cima(&mut self) {
        self.rect.y -= self.velocidade;
    }

    pub fn mover_para_baixo(&mut self) {
        self.rect.y += self.velocidade;
    }

    pub fn topo(&self) -> f32 {
        self.rect.top()
    }

    pub fn fundo(&self) -> f32 {
        self.rect.bottom()
    }
}
