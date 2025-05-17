use ggez::{
    conf::WindowMode,
    event::{self, EventHandler},
    graphics::{self, Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text, TextFragment},
    input::keyboard::{KeyCode, KeyboardContext},
    Context, ContextBuilder, GameResult,
};

struct MeuJogo {
    l_paddle: Rect,
    r_paddle: Rect,
    ball: Ball,
    l_score: u32,
    r_score: u32,
}

struct Ball {
    rect: Rect,
    x_vel: f32,
    y_vel: f32,
}

impl Ball {
    pub fn atualizar_posicao(&mut self){
        self.rect.x += self.x_vel;
        self.rect.y += self.y_vel;
    }
}

impl MeuJogo {
    pub fn new() -> Self {
        Self {
            l_paddle: Rect::new(50.0, 250.0, 10.0, 100.0),
            r_paddle: Rect::new(730.0, 250.0, 10.0, 100.0),
            ball: Ball {
                rect: Rect::new(390.0, 290.0, 20.0, 20.0),
                x_vel: 2.0,
                y_vel: 2.0,
            },
            l_score: 0,
            r_score: 0,
        }
    }

    pub fn controle_raquete(&mut self, velocidade: f32, ctx: &mut Context){

        let teclado = &ctx.keyboard;

        //controle da raquete esquerda

        if teclado.is_key_pressed(KeyCode::W) && !(self.l_paddle.y <= 0.0) {
            self.l_paddle.y -= velocidade;
        }
        if teclado.is_key_pressed(KeyCode::S) && !((self.l_paddle.y + self.l_paddle.h) >= 600.0) {
            self.l_paddle.y += velocidade;
        }

        //controle da raquede direita
        if teclado.is_key_pressed(KeyCode::Up) && !(self.r_paddle.y <= 0.0) {
            self.r_paddle.y -= velocidade;
        }
        if teclado.is_key_pressed(KeyCode::Down) && !((self.r_paddle.y + self.r_paddle.h) >= 600.0) {
            self.r_paddle.y += velocidade;
        }
    }

    pub fn angulo(&mut self, paddle : &Rect){
        let mut angle = (self.ball.rect.center().y - paddle.center().y)/ paddle.h / 2.0;
        if angle < 0.0{
            angle *= -1.0;
        }

        if self.ball.y_vel < 0.0 {
            self.ball.y_vel = (angle * 80.0) * self.ball.x_vel * - 1.0;
            return;
        }

        self.ball.y_vel = angle * self.ball.x_vel;
    }
}

impl EventHandler for MeuJogo {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const VELOCIDADE: f32 = 5.0;

        self.controle_raquete(VELOCIDADE, ctx); 

        // Colisão com as bordas e atualiza posição da bola
        if self.ball.rect.top() <= 0.0 || self.ball.rect.bottom() >= 600.0 {
            self.ball.y_vel = - self.ball.y_vel;
            self.ball.atualizar_posicao();
        } else {
            self.ball.atualizar_posicao();
        }

        // Colisão com as raquetes
            //raquete esquerda
        if self.ball.rect.overlaps(&self.l_paddle)
        {

            let distance = self.l_paddle.right() - self.ball.rect.left();
            self.ball.x_vel = -self.ball.x_vel;
            self.ball.rect.x += distance;
            let l_paddle = self.l_paddle.clone();
            self.angulo(&l_paddle);
        }
        
            //raquete direita
        if self.ball.rect.overlaps(& self.r_paddle)
        {
            let distance = self.r_paddle.left() - self.ball.rect.right();
            self.ball.x_vel = -self.ball.x_vel;
            self.ball.rect.x += distance;
            let r_paddle = self.r_paddle.clone();
            self.angulo(&r_paddle);
        }

        //reinicio da bola
        if self.ball.rect.left() <= 0.0 {
            self.r_score += 1;
            self.ball.rect.x = 390.0;
            self.ball.x_vel = 2.0;
            self.ball.y_vel = 2.0;
        } else if self.ball.rect.right() >= 800.0 {
            self.l_score += 1;
            self.ball.rect.x = 390.0;
            self.ball.x_vel = 2.0;
            self.ball.y_vel = 2.0;
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
