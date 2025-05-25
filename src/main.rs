use ggez::{
    conf::WindowMode,
    event::{self, EventHandler},
    Context, ContextBuilder, GameResult,
};

mod core {
    pub mod bola;
    pub mod raquete;
    pub mod pontuacao;
    pub mod traits;
    pub mod gerenciador;       // <<-- aqui
}

mod logica {
    pub mod fisica_bola;       // <<-- aqui
    pub mod fisica_raquete;    // <<-- e aqui
}

mod render;                     // pega render/mod.rs

use crate::core::gerenciador::GameManager;
use crate::render::desenhar_jogo;

struct MeuJogo {
    game: GameManager,
}

impl MeuJogo {
    pub fn new() -> Self {
        MeuJogo {
            game: GameManager::new(),
        }
    }
}

impl EventHandler for MeuJogo {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let size = ctx.gfx.window().inner_size();
        let (width, height) = (size.width as f32, size.height as f32);
        self.game.update(ctx, width, height)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        desenhar_jogo(ctx, &self.game)
    }
}

fn main() -> GameResult {
    let cb = ContextBuilder::new("pong_rust", "SeuNome")
        .window_mode(WindowMode::default().dimensions(800.0, 600.0));

    let (ctx, event_loop) = cb.build()?;
    let jogo = MeuJogo::new();
    event::run(ctx, event_loop, jogo)
}
