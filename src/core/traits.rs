use ggez::graphics::Rect;
use ggez::Context;
use ggez::input::keyboard::KeyCode;

pub trait Colisao{
    fn colisao_borda(& self, altura_tela: f32) -> bool;
}

pub trait MovimentacaoBola{
    fn rebater_borda(&mut self, altura_tela: f32);
    fn atualizar_posicao(&mut self);
    fn rebater_com_angulo(&mut self, raquete : &Rect);
    fn calcula_angulo(&self, raquete : &Rect) -> f32;
    fn reinicio_bola(&mut self, largura_tela: f32, altura_tela: f32);
    fn overlaps(& self, raquete: &Rect) -> bool;
    fn colisao_com_raquete(& mut self, raquete: &Rect);
}

pub trait Movimentacao {
    fn movimentacao(& mut self,ctx: &Context,velocidade: f32, up: KeyCode, down: KeyCode, altura_tela: f32);
}