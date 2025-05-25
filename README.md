# Pong
Um clássico jogo de Pong implementado em Rust, utilizando alguns princípios de engenharia de software:
- Responsabilidade Única
- Composição sobre Herança
- Segregação de Interface
- Lei de Demeter

## O Que é e Para Que Serve?
É um jogo onde se tem duas "raquetes" onde uma fica no lado esquerdo e a outra no direito e se tem uma bola que fica indo de um lado para o outro.
O objetivo é não deixar a bola passar da raquete.
Serve para passar o tempo.

## Principios:

### 1. Responsabilidade Única
Onde:
- src/core/bola.rs -> Cuida da definição de bola.
- src/logica/fisica_bola.rs -> Lógica de movimento/colisão da bola.   


Problema Resolvido:
Evita que a struct Bola seja responsável por múltiplas tarefas (como desenho, entrada e regras de negócio). Cada módulo foca em uma única responsabilidade.

---------------------------------------------------

### 2. Composição sobre Herança
Onde:

- src/core/traits.rs

pub trait Geometria {
    fn top(&self) -> f32;
    fn bottom(&self) -> f32;
    ...
}

- src/logica/fisica_bola.rs

impl Geometria for Bola {
    fn top(&self) -> f32 { self.rect.top() }
    fn bottom(&self) -> f32 { self.rect.bottom() }
    ...
}


Problema Resolvido:
Permite adicionar funções como top() ou bottom() usando interfaces(traits) sem modificar a estrutura original.

---------------------------------------------------

### 3. Segregação de Interface

Onde:

- src/core/traits.rs

pub trait MovimentacaoBola {
    fn mover(&mut self);
}

- src/logica/fisica_bola.rs

impl MovimentacaoBola for Bola {
    fn mover(&mut self) {
        self.rect.x += self.velocidade.x;
        self.rect.y += self.velocidade.y;
    }
}


Problema Resolvido:
Evita que a struct Bola precise implementar métodos que só fazem sentido para a Raquete, e vice-versa.

---------------------------------------------------

### 4. Lei de Demeter

Onde:

- src/core/traits.rs

pub trait Geometria {
    fn top(&self) -> f32;
    // ...
}

- src/logica/fisica_bola.rs

impl Geometria for Bola {
    fn top(&self) -> f32 { self.rect.top() }
}


Problema Resolvido:
Evita que precise acessar ball.rect.top() e acesse apenas ball.top().
