use ggez::graphics::Rect;
use rand::Rng;

use crate::core::bola::Bola;
use crate::core::raquete::LadoRaquete;
use crate::core::traits::{Colisao, MovimentacaoBola, Geometria};

impl Colisao for Bola{
    fn colisao_borda(& self, altura_tela: f32) -> bool{
        self.top() <= 0.0 || self.bottom() >= altura_tela
    }
}

impl MovimentacaoBola for Bola {
    fn rebater_borda(&mut self, altura_tela: f32){
        if self.colisao_borda(altura_tela) {
            self.y_vel = - self.y_vel;
        }
    }

    fn calcula_angulo(&self, raquete : &Rect) -> f32{
        let angulo = (self.rect.center().y - raquete.center().y) / (raquete.h/2.0);

        angulo.clamp(-1.0, 1.0)
    }

    fn rebater_com_angulo(&mut self, raquete : &Rect){
        let angulo = self.calcula_angulo(raquete);
        let mut max_y_vel = 2.0 * self.x_vel;
        if self.rect.x > 300.0 {
            max_y_vel *= -1.0;
        }
        self.y_vel = angulo * max_y_vel;
    }

    fn atualizar_posicao(&mut self){
        self.mover();
    }

    fn reinicio_bola(&mut self, largura_tela: f32, altura_tela: f32){
        let mut rng = rand::thread_rng();
        let x_dir = if rng.gen_bool(0.5) {1.0} else {-1.0};
        let y_dir = if rng.gen_bool(0.5) {1.0} else {-1.0};

        self.x_vel = x_dir * 2.0;
        self.y_vel = y_dir * 2.0;

        self.rect.x = (largura_tela - self.rect.w) / 2.0;
        self.rect.y = (altura_tela - self.rect.h) / 2.0;
    }

    fn overlaps(& self, raquete: &Rect) -> bool{
        self.rect.overlaps(&raquete)
    }

    fn colisao_com_raquete(& mut self, raquete: &Rect, lado:&LadoRaquete){
        let distance = match lado {
            LadoRaquete::E => raquete.right() - self.left(),
            LadoRaquete::D => raquete.left() - self.right(),
        };

        self.x_vel = -self.x_vel * 1.05;
        self.rect.x += distance;
        self.rebater_com_angulo(raquete);
    }
}

impl Geometria for Bola{
    fn top(&self) -> f32{
        self.rect.top()
    }

    fn bottom(&self) -> f32{
        self.rect.bottom()
    }

    fn left(&self) -> f32{
        self.rect.left()
    }

    fn right(&self) -> f32{
        self.rect.right()
    }
}