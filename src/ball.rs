use ggez::{
    graphics::Rect,
};

use crate::colisao::Colisao;
pub struct Ball {
    pub rect: Rect,
    pub x_vel: f32,
    pub y_vel: f32,
}

impl Colisao for Ball{
    fn colisao_borda(& self, altura_tela: f32) -> bool{
        self.rect.top() <= 0.0 || self.rect.bottom() >= altura_tela
    }
}

impl Ball {
    pub fn new(ball: Rect, x_vel: f32, y_vel: f32) -> Self {
        Ball {
            rect: ball,
            x_vel: x_vel,
            y_vel: y_vel,
        }
    }

    pub fn rebater_borda(&mut self, altura_tela: f32){
        if self.colisao_borda(altura_tela) {
            self.y_vel = - self.y_vel;
        }
    }

    pub fn atualizar_posicao(&mut self){
        self.rect.x += self.x_vel;
        self.rect.y += self.y_vel;
    }

    fn calcula_angulo(&self, paddle : &Rect) -> f32{
        let mut angle = (self.rect.center().y - paddle.center().y) / paddle.h;

        angle
    }

    pub fn rebater_com_angulo(&mut self, paddle : &Rect){
        let angle = self.calcula_angulo(paddle);


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

    pub fn reinicio_ball(&mut self, largura_tela: f32, altura_tela: f32){
        self.x_vel = 2.0;
        self.y_vel = 2.0;
        self.rect.x = largura_tela/2.0;
        self.rect.y = altura_tela/2.0;
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
}