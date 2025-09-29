use rand::Rng;

pub struct Food {
    pub x: u16,
    pub y: u16,
    pub nutrients: f64,
}

impl Food {
    pub fn random(x: u16, y: u16) -> Self {
        Self {
            x,
            y,
            nutrients: random_nutrients(),
        }
    }
}

fn random_nutrients() -> f64 {
    let mut rng = rand::rng();

    let speed: f64 = rng.gen_range(0.0..100.0);

    speed
}
