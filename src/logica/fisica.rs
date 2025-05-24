use ggez::{Context};
use ggez::input::keyboard::KeyCode;
use ggez::graphics::Rect;

use crate::core::bola::Bola;
use crate::core::raquete::Raquete;
use crate::core::traits::{Colisao, MovimentacaoBola, Movimentacao};

impl Colisao for Bola{
    fn colisao_borda(& self, altura_tela: f32) -> bool{
        self.rect.top() <= 0.0 || self.rect.bottom() >= altura_tela
    }
}

impl MovimentacaoBola for Bola {
    fn rebater_borda(&mut self, altura_tela: f32){
        if self.colisao_borda(altura_tela) {
            self.y_vel = - self.y_vel;
        }
    }

    fn calcula_angulo(&self, raquete : &Rect) -> f32{
        let angle = (self.rect.center().y - raquete.center().y) / raquete.h;

        angle
    }

    fn rebater_com_angulo(&mut self, raquete : &Rect){
        let angle = self.calcula_angulo(raquete);

        if angle == 0.0{
            self.y_vel = 0.0;
        } else if angle < 0.0 {
            if angle >= -0.25 {
                self.y_vel = -self.x_vel * 0.5;
            } else if angle >= -0.50 {
                self.y_vel = -self.x_vel * 1.0;
            } else if angle >= -0.75 {
                self.y_vel = -self.x_vel * 1.5;
            } else if angle >= -1.00 {
                self.y_vel = -self.x_vel * 2.0;
            }
        } else if angle > 0.0 {
            if angle <= 0.25 {
                self.y_vel = self.x_vel * 0.5;
            } else if angle <= 0.50 {
                self.y_vel = self.x_vel * 1.0;
            } else if angle <= 0.75 {
                self.y_vel = self.x_vel * 1.5;
            } else if angle <= 1.00 {
                self.y_vel = self.x_vel * 2.0;
            }
        }
    }

    fn atualizar_posicao(&mut self){
        self.rect.x += self.x_vel;
        self.rect.y += self.y_vel;
    }

    fn reinicio_bola(&mut self, largura_tela: f32, altura_tela: f32){
        self.x_vel = 2.0;
        self.y_vel = 2.0;
        self.rect.x = largura_tela/2.0;
        self.rect.y = altura_tela/2.0;
    }

    fn overlaps(& self, raquete: &Rect) -> bool{
        self.rect.overlaps(&raquete)
    }

    fn colisao_com_raquete(& mut self, raquete: &Rect){
        let distance = raquete.left() - self.rect.right();
        self.x_vel = -self.x_vel;
        self.rect.x += distance;
        self.rebater_com_angulo(raquete);
    }
}

impl Movimentacao for Raquete{
    fn movimentacao(& mut self,ctx: &Context,velocidade: f32, up: KeyCode, down: KeyCode, altura_tela: f32){
        if ctx.keyboard.is_key_pressed(up) && self.rect.top() >= 0.0 {
            self.rect.y -= velocidade;
        }
        if ctx.keyboard.is_key_pressed(down) && self.rect.bottom() <= altura_tela {
            self.rect.y += velocidade;
        }
    }
}