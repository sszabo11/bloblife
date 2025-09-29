use rand::Rng;

const MUTATION_RATE: f64 = 0.0001;

pub struct Creature {
    pub x: u16,
    pub y: u16,
    pub smell: f64,
    pub speed: f64,
    pub energy: f64,
    pub age: f64,
    pub wander_factor: f64,
    pub size: f64,
}

impl Creature {
    pub fn random(x: u16, y: u16) -> Self {
        Self {
            x,
            y,
            smell: random_smell(),
            age: 30.00,
            speed: random_speed(),
            energy: random_energy(),
            wander_factor: random_wander(),
            size: random_size(),
        }
    }

    pub fn reproduce(parents: (&Creature, &Creature)) -> Self {
        let mut rng = rand::rng();
        let speed_mut: f64 = rng.gen_range(0.00..1.00);
        let size_mut: f64 = rng.gen_range(0.00..1.00);
        //let smell_mut: f64 = rng.gen_range(0.60..0.7);

        let new_speed = (parents.0.speed + parents.1.speed) / 2.00 * speed_mut;
        let new_size = (parents.0.size + parents.1.size) / 2.00 * size_mut;

        let mutation = rng.gen_range(-MUTATION_RATE..=MUTATION_RATE);
        let mut new_smell = (parents.0.smell + parents.1.smell) / 2.00 + mutation;

        new_smell = new_smell.max(0.0).min(1.0);
        let new_x = (parents.0.x + parents.1.x) / 2;
        let new_y = (parents.0.y + parents.1.y) / 2;

        Self {
            x: new_x,
            y: new_y,
            age: 0.00,
            smell: new_smell,
            speed: new_speed,
            energy: 100.00,
            wander_factor: random_wander(),
            size: new_size,
        }
    }
}

fn random_speed() -> f64 {
    let mut rng = rand::rng();

    let speed: f64 = rng.gen_range(0.0..100.0);

    speed
}

fn random_energy() -> f64 {
    let mut rng = rand::rng();

    let energy: f64 = rng.gen_range(0.0..100.0);

    energy
}

fn random_wander() -> f64 {
    let mut rng = rand::rng();

    let wander: f64 = rng.gen_range(0.0..100.0);

    wander
}

fn random_size() -> f64 {
    let mut rng = rand::rng();

    let size: f64 = rng.gen_range(0.0..3.0);

    size
}

fn random_smell() -> f64 {
    let mut rng = rand::rng();

    let smell: f64 = rng.gen_range(0.0..2.0);

    smell
}
