pub enum GameLength {
    Tonpuusen,
    Hanchan,
}

pub struct Rules {
    pub game_length: GameLength,
    pub aka_ari: bool,
    pub kuitan_ari: bool,
    // TODO more rules
}
