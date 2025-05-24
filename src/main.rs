use ggez::{
    conf::WindowMode, context, event::{self, EventHandler}, graphics::{self, Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text, TextFragment, GraphicsContext}, input::keyboard::KeyCode, Context, ContextBuilder, GameResult
};

mod colisao;
mod ball;
use crate::ball::Ball;

mod raquete;
use crate::raquete::Raquete;

struct MeuJogo {
    l_paddle: Rect,
    r_paddle: Rect,
    ball: Ball,
    l_score: u32,
    r_score: u32,
}

impl MeuJogo {
    pub fn new() -> Self {
        let rect_ball = Rect::new(390.0, 290.0, 20.0, 20.0);
        Self {
            l_paddle: Rect::new(50.0, 250.0, 10.0, 100.0),
            r_paddle: Rect::new(730.0, 250.0, 10.0, 100.0),
            ball: Ball::new(rect_ball, 2.0, 2.0),
            l_score: 0,
            r_score: 0,
        }
    }
}

impl EventHandler for MeuJogo {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const VELOCIDADE: f32 = 5.0;

        let size = ctx.gfx.window().inner_size();
        let (width, height) = (size.width as f32, size.height as f32);


        self.l_paddle.movimentacao(ctx, VELOCIDADE, KeyCode::W, KeyCode::S, height);
        self.r_paddle.movimentacao(ctx, VELOCIDADE, KeyCode::Up, KeyCode::Down, height);

        // Colisão com as bordas e atualiza posição da bola
        self.ball.rebater_borda(height);
        self.ball.atualizar_posicao();

        // Colisão com as raquetes
            //raquete esquerda
        if self.ball.rect.overlaps(&self.l_paddle)
        {

            let distance = self.l_paddle.right() - self.ball.rect.left();
            self.ball.x_vel = -self.ball.x_vel;
            self.ball.rect.x += distance;
            let l_paddle = self.l_paddle.clone();
            self.ball.rebater_com_angulo(&l_paddle); 
        }
        
            //raquete direita
        if self.ball.rect.overlaps(& self.r_paddle)
        {
            let distance = self.r_paddle.left() - self.ball.rect.right();
            self.ball.x_vel = -self.ball.x_vel;
            self.ball.rect.x += distance;
            let r_paddle = self.r_paddle.clone();
            self.ball.rebater_com_angulo(&r_paddle);
        }

        //reinicio da bola
        if self.ball.left() <= 0.0 {
            self.r_score += 1;
            self.ball.reinicio_ball(width, height);
        } else if self.ball.right() >= width {
            self.l_score += 1;
            self.ball.reinicio_ball(width, height);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Cria um novo canvas para desenhar
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        // Fazer a raquete esquerda
        let l_paddle_mesh =
            Mesh::new_rectangle(ctx, DrawMode::fill(), self.l_paddle, Color::WHITE)?;
        canvas.draw(&l_paddle_mesh, graphics::DrawParam::default());

        // Fazer a raquete direita
        let r_paddle_mesh =
            Mesh::new_rectangle(ctx, DrawMode::fill(), self.r_paddle, Color::WHITE)?;
        canvas.draw(&r_paddle_mesh, DrawParam::default());

        // Fazer a bola
        let ball_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), self.ball.rect, Color::WHITE)?;
        canvas.draw(&ball_mesh, DrawParam::default());

        // Fazer o placar
        let placar = format!("{:<5}{:>5}", self.l_score, self.r_score);
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
