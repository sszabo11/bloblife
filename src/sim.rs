use crate::creatures::Creature;
use crate::food;
use crate::food::Food;
use crate::map;
use crossterm::ExecutableCommand;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style;
use crossterm::style::Print;
use crossterm::style::StyledContent;
use crossterm::style::Stylize;
use crossterm::terminal;
use crossterm::terminal::ClearType;
use crossterm::terminal::ScrollUp;
use crossterm::terminal::SetSize;
use crossterm::{QueueableCommand, cursor};
use rand::Rng;
use std::io;
use std::io::{Write, stdout};

const MIN_ENERGY_TO_REPRODUCE: f64 = 50.00;
const REPRODUCTION_RANGE: f64 = 50.00;
const FOOD_PER_FRAME: u16 = 3;

pub struct Simulation {
    pub map: map::Map,
    pub died: u64,
    pub born: u64,
    pub food: Vec<Food>,
    pub creatures: Vec<Creature>,
    pub framerate: u64,
}

impl Simulation {
    pub fn new(width: u16, height: u16, framerate: u64) -> Self {
        Self {
            died: 0,
            born: 0,
            framerate,
            food: Vec::new(),
            map: map::Map::new(width, height),
            creatures: Vec::new(),
        }
    }

    pub fn seed(&mut self, creatures: usize, food: usize) {
        for _n in 0..creatures {
            let mut rng = rand::rng();

            let x = rng.gen_range(0..self.map.width);
            let y = rng.gen_range(0..self.map.height);

            let c = Creature::random(x, y);
            self.creatures.push(c)
        }

        for _n in 0..food {
            let mut rng = rand::rng();

            let x = rng.gen_range(0..self.map.width);
            let y = rng.gen_range(0..self.map.height);

            let c = Food::random(x, y);
            self.food.push(c)
        }
    }
    pub fn move_creatures(&mut self) {
        let map_width = self.map.width;
        let map_height = self.map.height;

        for c in &mut self.creatures {
            let map_size = f64::from(map_width * map_height).sqrt(); // Calculate Map Size once per creature (or once outside the loop)
            let smell_range = map_size * c.smell;

            // 1. Find the nearest food that is within the creature's smell range
            let nearest_food = self
                .food
                .iter()
                .map(|f| {
                    let distance = calculate_distance(c.x, c.y, f.x, f.y);
                    (f, distance) // Pair the food reference with its distance
                })
                // Filter to keep only food that is within the creature's smell range
                .filter(|&(_, distance)| distance <= smell_range)
                // Find the food with the minimum distance
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            // 2. Execute a single movement based on the discovery
            if let Some((food, _distance)) = nearest_food {
                // Found nearby food: move towards the nearest one
                move_towards_food(c, food, map_width, map_height);
            } else {
                // No food in range: move randomly
                move_randomly(map_width, map_height, c);
            }
        }
    }

    pub fn move_creatures2(&mut self) {
        for c in &mut self.creatures {
            //c.x = (c.x + 1) % self.map.width;
            //c.y = (c.y + 1) % self.map.height;

            for f in &self.food {
                //let d = (((c.y - f.y).pow(2) + (c.x - f.x).pow(2)) as f64).sqrt();

                let d = calculate_distance(c.x, c.y, f.x, f.y);
                let map_size = f64::from(self.map.width * self.map.height).sqrt();
                let range = map_size * c.smell;

                if d <= range {
                    move_towards_food(c, f, self.map.width, self.map.height);
                } else {
                    move_randomly(self.map.width, self.map.height, c);
                }
            }
        }
    }

    pub fn logic(&mut self) {
        self.eat()
    }

    fn eat(&mut self) {
        self.food.retain(|f| {
            !self.creatures.iter_mut().any(|c| {
                if c.x == f.x && c.y == f.y {
                    c.energy += 50.00;
                    return true;
                }
                false
            })
        });
    }

    fn age(&mut self) {
        let mut rng = rand::rng();
        for c in &mut self.creatures {
            c.age += 1.00;
        }

        self.creatures.retain(|c| {
            let death_age = rng.gen_range(60.00..100.00);

            if c.age < death_age {
                return true;
            }
            self.died += 1;
            false
        });
    }
    fn create_food(&mut self) {
        let mut rng = rand::rng();

        for _n in 0..FOOD_PER_FRAME {
            let x = rng.gen_range(0..self.map.width);
            let y = rng.gen_range(0..self.map.height);
            self.food.push(Food::random(x, y))
        }
    }

    pub fn lifecycle(&mut self, ticks: u64) {
        if ticks.is_multiple_of(10) {
            self.age();
            self.create_food();
            self.handle_death();
        }

        //let mut new_creatures: Vec<usize> = Vec::new();
        let creature_count = self.creatures.len();

        let mut parents_to_mutate = Vec::<(usize, usize)>::new();

        for i in 0..creature_count {
            if self.creatures[i].energy < MIN_ENERGY_TO_REPRODUCE || self.creatures[i].age < 20.00 {
                continue;
            }
            for j in (i + 1)..creature_count {
                if self.creatures[j].energy < MIN_ENERGY_TO_REPRODUCE
                    || self.creatures[j].age < 20.00
                {
                    continue;
                }
                let c1 = &self.creatures[i];
                let c2 = &self.creatures[j];

                //if c2.energy < MIN_ENERGY_TO_REPRODUCE {
                //    continue;
                //}

                let distance = calculate_distance(c1.x, c1.y, c2.x, c2.y);

                if distance <= REPRODUCTION_RANGE {
                    // Store the indices and break from the inner loop
                    parents_to_mutate.push((i, j));
                    break;
                }
                //if distance <= REPRODUCTION_RANGE {
                //    c1.energy /= 2.0;
                //    c2.energy /= 2.0;

                //    let child = Creature::reproduce((c1, c2));

                //    self.creatures.push(child);

                //    break;
                //}
            }
        }

        for (i, j) in parents_to_mutate {
            //self.creatures[i].age += 1;
            //self.creatures[j].age += 1;

            //if self.creatures[i].age < 20 || self.creatures[j].age < 20 {
            //    break;
            //}
            self.creatures[i].energy /= 2.0;
            self.creatures[j].energy /= 2.0;

            let child = Creature::reproduce((&self.creatures[i], &self.creatures[j]));

            self.born += 1;
            self.creatures.push(child);
        }
    }

    pub fn handle_death(&mut self) {
        for c in &mut self.creatures {
            if c.energy > 0.00 {
                c.energy -= 5.00;
            }
        }

        self.creatures.retain(|c| {
            if c.energy > 0.00 {
                return true;
            }
            self.died += 1;
            false
        });
    }

    pub fn draw(&mut self) {
        let mut stdout = stdout();

        for y in 0..self.map.height {
            for x in 0..self.map.width {
                //if (x.is_multiple_of(2)) {
                stdout.queue(cursor::MoveTo(x, y)).unwrap();

                if self.creatures.iter().any(|c| c.x == x && c.y == y) {
                    //print!("🦆");
                    stdout.queue(Print("o".blue())).unwrap();
                    //stdout.queue(Print("🦆")).unwrap();
                } else if self.food.iter().any(|c| c.x == x && c.y == y) {
                    //print!("🍌");
                    stdout.queue(Print("*".red())).unwrap();
                    //stdout.queue(Print("🍒")).unwrap();
                } else {
                    //print!("");
                    stdout.queue(Print(" ")).unwrap();
                    //stdout.queue(Print("  ")).unwrap();
                }
                //} else {
                //    //stdout.queue(Print(" ")).unwrap();
                //}
            }
        }
    }

    pub fn draw_dead(&self) {
        let mut stdout = stdout();
        stdout.queue(terminal::Clear(ClearType::All)).unwrap();
        let center_x = self.map.width / 2;
        let center_y = self.map.height / 2;

        let message = "R.I.P";
        let message2 = "All the creatures have died 💀.";

        stdout
            .queue(cursor::MoveTo(center_x - 5, center_y))
            .unwrap();

        stdout.queue(style::Print(message)).unwrap();

        stdout
            .queue(cursor::MoveTo(center_x - 20, center_y + 1))
            .unwrap();
        stdout.queue(style::Print(message2)).unwrap();
    }

    pub fn draw_status(&self, ticks: u64) {
        let mut stdout = stdout();

        // Define the row where the status text begins (e.g., just below the map)
        let status_row_start = self.map.height;

        let total_energy: f64 = self.creatures.iter().map(|c| c.energy).sum();
        let avg_energy = total_energy / self.creatures.len() as f64;

        let total_smell: f64 = self.creatures.iter().map(|c| c.smell).sum();
        let avg_smell = total_smell / self.creatures.len() as f64;

        let total_age: f64 = self.creatures.iter().map(|c| c.age).sum();
        let avg_age = total_age / self.creatures.len() as f64;

        // Line 1: Summary Stats
        let status_line_1 = format!(
            "Generation: {} | Creatures: {} | Food: {} | Avg energy: {} | Avg smell: {} | Avg age: {} | Died: {} | Born: {}",
            ticks,
            self.creatures.len(),
            self.food.len(),
            avg_energy,
            avg_smell,
            avg_age,
            self.died,
            self.born,
        );

        // Line 2: Instructions
        let status_line_2 = "Press ESC to Quit. Speed (Logic/Draw): 1ms/100ms";

        // Draw Line 1
        //stdout.queue(cursor::MoveTo(2, status_row_start)).unwrap();
        //stdout
        //    .queue(terminal::Clear(ClearType::CurrentLine))
        //    .unwrap(); // Clear the line first
        //stdout.queue(style::Print(status_line_1)).unwrap();

        // Draw Line 2
        stdout
            .queue(cursor::MoveTo(0, status_row_start + 1))
            .unwrap();
        stdout
            .queue(terminal::Clear(ClearType::CurrentLine))
            .unwrap(); // Clear the line first
        stdout.queue(style::Print(status_line_1)).unwrap();
    }
}

fn move_towards_food(creature: &mut Creature, food: &Food, map_width: u16, map_height: u16) {
    let dx = food.x as i32 - creature.x as i32;
    let dy = food.y as i32 - creature.y as i32;

    let mut new_x = creature.x as i32;
    let mut new_y = creature.y as i32;

    if dx.abs() > dy.abs() {
        // Move primarily on the X-axis (horizontal)
        if dx > 0 {
            new_x += 1;
        } else if dx < 0 {
            new_x -= 1;
        }
    } else {
        // Move primarily on the Y-axis (vertical), or if distances are equal
        if dy > 0 {
            new_y += 1;
        } else if dy < 0 {
            new_y -= 1;
        }
    }

    // Apply Wrap-Around Boundary Logic (crucial for safety)
    let w = map_width as i32;
    let h = map_height as i32;
    creature.x = ((new_x % w + w) % w) as u16;
    creature.y = ((new_y % h + h) % h) as u16;
    //// Move 1 unit in the X direction
    //if dx > 0 {
    //    new_x += 1;
    //} else if dx < 0 {
    //    new_x -= 1;
    //}

    //// Move 1 unit in the Y direction (FIXED TYPO HERE: changed 'dx' to 'dy')
    //if dy > 0 {
    //    new_y += 1;
    //} else if dy < 0 {
    //    // This condition MUST check 'dy', not 'dx'
    //    new_y -= 1;
    //}

    //creature.x = new_x as u16;
    //creature.y = new_y as u16;
}

fn calculate_distance(x1: u16, y1: u16, x2: u16, y2: u16) -> f64 {
    let dx = x1 as i32 - x2 as i32;
    let dy = y1 as i32 - y2 as i32;

    let dx_squared = (dx as f64).powi(2);
    let dy_squared = (dy as f64).powi(2);

    (dx_squared + dy_squared).sqrt()
}
fn move_randomly(map_width: u16, map_height: u16, creature: &mut Creature) {
    let mut rng = rand::rng();
    let dx: i32;
    let dy: i32;

    let w = map_width as i32;
    let h = map_height as i32;

    loop {
        // Generate dx and dy, each between -1, 0, and 1.
        let new_dx = rng.gen_range(-1..=1);
        let new_dy = rng.gen_range(-1..=1);

        // If either one is non-zero, the loop breaks.
        if new_dx != 0 || new_dy != 0 {
            dx = new_dx;
            dy = new_dy;
            break;
        }
    }

    let new_x = creature.x as i32 + dx;
    let new_y = creature.y as i32 + dy;

    creature.x = ((new_x % w + w) % w) as u16;
    creature.y = ((new_y % h + h) % h) as u16;
}
