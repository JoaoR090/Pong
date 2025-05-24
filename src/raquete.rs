use ggez::{
    graphics::Rect,
    input::keyboard::KeyCode,
    Context,
};

pub trait Raquete {
    fn movimentacao(& mut self,ctx: &Context,velocidade: f32, up: KeyCode, down: KeyCode, altura_tela: f32);
}

impl Raquete for Rect{
    fn movimentacao(& mut self,ctx: &Context,velocidade: f32, up: KeyCode, down: KeyCode, altura_tela: f32){
        if ctx.keyboard.is_key_pressed(up) && self.top() >= 0.0 {
            self.y -= velocidade;
        }
        if ctx.keyboard.is_key_pressed(down) && self.bottom() <= altura_tela {
            self.y += velocidade;
        }
    }
}