use crate::core::bola::Bola;
use crate::core::pontuacao::{Lado, Pontuacao};
use crate::core::raquete::{LadoRaquete, Raquete};
use crate::core::traits::{MovimentacaoBola, MovimentacaoRaquete};
use crate::core::traits::Geometria;
use ggez::{input::keyboard::KeyCode, Context, GameResult};

pub struct GameManager {
    pub bola: Bola,
    pub e_raquete: Raquete,
    pub d_raquete: Raquete,
    pub pontuacao: Pontuacao,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            e_raquete: Raquete::new(50.0, 250.0, 10.0, 100.0, 5.0, LadoRaquete::E),
            d_raquete: Raquete::new(730.0, 250.0, 10.0, 100.0, 5.0, LadoRaquete::D),
            bola: Bola::new(20.0, 20.0, 2.0, 2.0),
            pontuacao: Pontuacao::new(0, 0),
        }
    }

    pub fn update(&mut self, ctx: &mut Context, largura: f32, altura: f32) -> GameResult {
        // Movimentação de Raquete
        self.e_raquete.movimentacao(ctx, KeyCode::W, KeyCode::S, altura);
        self.d_raquete.movimentacao(ctx, KeyCode::Up, KeyCode::Down, altura);

        // Movimentação de Bola
        self.bola.rebater_borda(altura);
        self.bola.atualizar_posicao();

        // Colisão com raquetes
        for raquete in [&self.e_raquete, &self.d_raquete] {
            if self.bola.overlaps(&raquete.rect) {
                self.bola.colisao_com_raquete(&raquete.rect, &raquete.lado);
            }
        }

        // Pontuação
        if self.bola.left() <= 0.0 {
            self.pontuacao.atulizar_pontuacao(Lado::E);
            self.bola.reinicio_bola(largura, altura);
        } else if self.bola.right() >= largura {
            self.pontuacao.atulizar_pontuacao(Lado::D);
            self.bola.reinicio_bola(largura, altura);
        }

        Ok(())
    }
}
