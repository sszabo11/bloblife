// Size
pub struct Map {
    pub width: u16,
    pub height: u16,
}

impl Map {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}
