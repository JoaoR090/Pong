use crate::core::gerenciador::GameManager;
use ggez::{
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Text, TextFragment},
    Context, GameResult,
};

pub fn desenhar_jogo(ctx: &mut Context, game: &GameManager) -> GameResult {
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

    let e_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), game.e_raquete.rect, Color::WHITE)?;
    canvas.draw(&e_mesh, DrawParam::default());

    let d_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), game.d_raquete.rect, Color::WHITE)?;
    canvas.draw(&d_mesh, DrawParam::default());

    let bola_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), game.bola.get_rect(), Color::WHITE)?;
    canvas.draw(&bola_mesh, DrawParam::default());

    let (esq, dir) = game.pontuacao.get();
    let texto = Text::new(TextFragment::new(format!("{:<5}{:>5}", esq, dir)).scale(30.0));
    canvas.draw(&texto, DrawParam::default().dest([350.0, 20.0]));

    canvas.finish(ctx)
}
