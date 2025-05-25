use ggez::Context;
use ggez::input::keyboard::KeyCode;

use crate::core::raquete::{Raquete};
use crate::core::traits::{MovimentacaoRaquete};

impl MovimentacaoRaquete for Raquete{
    fn movimentacao(& mut self,ctx: &Context, up: KeyCode, down: KeyCode, altura_tela: f32){
        if ctx.keyboard.is_key_pressed(up) && self.topo() >= 0.0 {
            self.mover_para_cima();
        }
        if ctx.keyboard.is_key_pressed(down) && self.fundo() <= altura_tela {
            self.mover_para_baixo();
        }
    }
}