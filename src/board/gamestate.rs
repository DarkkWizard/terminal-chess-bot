pub struct Gamestate {
    pub side_to_play: u8,
}
impl Gamestate {
    pub fn new(side: u8) -> Gamestate {
        Gamestate { side_to_play: side }
    }
}
