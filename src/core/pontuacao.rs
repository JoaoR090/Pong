pub struct Pontuacao {
    p_e: u32,
    p_d: u32,
}

pub enum Lado{
    E,
    D,
}

impl Pontuacao {
    pub fn new(e: u32, d: u32) -> Self {
        Pontuacao{
            p_e: e,
            p_d: d,
        }
    }

    pub fn atulizar_pontuacao(&mut self, lado: Lado){
        match lado {
            Lado::E => self.p_e += 1,
            Lado::D => self.p_d += 1,
        }
    }

    pub fn get(&self) -> (u32, u32){
        (self.p_e, self.p_d)
    }
}