use ggez::{
    conf::WindowMode, event::{self, EventHandler}, graphics::{self, Canvas, Color, DrawMode, DrawParam, Mesh, Text, TextFragment}, input::keyboard::KeyCode, Context, ContextBuilder, GameResult
};

mod core {
    pub mod bola;
    pub mod raquete;
    pub mod traits;
}
mod logica{
    pub mod fisica;
}

use crate::core::bola::Bola;
use crate::core::traits::Movimentacao;
use crate::core::traits::MovimentacaoBola;
use crate::core::raquete::Raquete;


struct MeuJogo {
    e_raquete: Raquete,
    d_raquete: Raquete,
    bola: Bola,
    e_score: u32,
    d_score: u32,
}

impl MeuJogo {
    pub fn new() -> Self {
        Self {
            e_raquete: Raquete::new(50.0, 250.0, 10.0, 100.0, 5.0),
            d_raquete: Raquete::new(730.0, 250.0, 10.0, 100.0, 5.0),
            bola: Bola::new(20.0, 20.0, 2.0, 2.0),
            e_score: 0,
            d_score: 0,
        }
    }
}

impl EventHandler for MeuJogo {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const VELOCIDADE: f32 = 5.0;

        let size = ctx.gfx.window().inner_size();
        let (width, height) = (size.width as f32, size.height as f32);


        self.e_raquete.movimentacao(ctx, VELOCIDADE, KeyCode::W, KeyCode::S, height);
        self.d_raquete.movimentacao(ctx, VELOCIDADE, KeyCode::Up, KeyCode::Down, height);

        // Colisão com as bordas e atualiza posição da bola
        self.bola.rebater_borda(height);
        self.bola.atualizar_posicao();

        // Colisão com as raquetes
            //raquete esquerda
        if self.bola.overlaps(&self.e_raquete.rect)
        {
            self.bola.colisao_com_raquete(& self.e_raquete.rect);
        }
        
            //raquete direita
        if self.bola.overlaps(& self.d_raquete.rect)
        {
            self.bola.colisao_com_raquete(& self.d_raquete.rect);
        }

        //reinicio da bola e atualização de placar
        if self.bola.left() <= 0.0 {
            self.d_score += 1;
            self.bola.reinicio_bola(width, height);
        } else if self.bola.right() >= width {
            self.e_score += 1;
            self.bola.reinicio_bola(width, height);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Cria um novo canvas para desenhar
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        // Fazer a raquete esquerda
        let e_raquete_mesh =
            Mesh::new_rectangle(ctx, DrawMode::fill(), self.e_raquete.rect, Color::WHITE)?;
        canvas.draw(&e_raquete_mesh, graphics::DrawParam::default());

        // Fazer a raquete direita
        let d_raquete_mesh =
            Mesh::new_rectangle(ctx, DrawMode::fill(), self.d_raquete.rect, Color::WHITE)?;
        canvas.draw(&d_raquete_mesh, DrawParam::default());

        // Fazer a bola
        let bola_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), self.bola.rect(), Color::WHITE)?;
        canvas.draw(&bola_mesh, DrawParam::default());

        // Fazer o placar
        let placar = format!("{:<5}{:>5}", self.e_score, self.d_score);
        let texto = Text::new(TextFragment::new(placar).scale(30.0));
        canvas.draw(&texto, DrawParam::default().dest([350.0, 20.0]));

        // Finaliza e apresenta o desenho na tela
        canvas.finish(ctx)?;

        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ContextBuilder::new("music_dash", "Eletric_God")
        .window_mode(WindowMode::default().dimensions(800.0, 600.0));

    let (ctx, event_loop) = cb.build()?;
    let jogo = MeuJogo::new();
    event::run(ctx, event_loop, jogo)
}
