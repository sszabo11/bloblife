mod creatures;
mod food;
mod map;
mod render;
mod sim;
use crossterm::terminal::size;
use crossterm::{ExecutableCommand, cursor, style::Print, terminal};
use map::Map;
use sim::Simulation;
use std::io::{self, Write, stdout};
use std::thread::sleep;
use std::time::Duration;

fn main() -> io::Result<()> {
    println!("Starting simulation...");

    println!("--------------- Welcome to Blobworld! ---------------");

    //let map = Map::new(100);

    let (cols, rows) = size().unwrap();

    let mut simulation = Simulation::new(cols, rows, 1);

    simulation.seed(100, 1000);

    render::render(simulation)?;

    Ok(())
}
