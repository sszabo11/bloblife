use std::{
    io::{self, Write, stdout},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::{Print, Stylize},
    terminal::{self, size},
};

use crate::sim::Simulation;

pub fn render(mut sim: Simulation) -> io::Result<()> {
    let mut stdout = stdout();

    terminal::enable_raw_mode().unwrap();

    execute!(
        stdout,
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    )
    .unwrap();

    let (cols, rows) = size()?;

    let mut running = true;
    let mut dead = true;
    const DRAW_INTERVAL: u64 = 1;
    let mut ticks: u64 = 0;

    while running {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc {
                    running = false;
                }
            }
        }

        sim.move_creatures();
        sim.logic();
        sim.lifecycle(ticks);

        if ticks % DRAW_INTERVAL == 0 {
            sim.draw();
            sim.draw_status(ticks);
            //stdout.queue(cursor::MoveTo(cols, rows))?.queue(Print(
            //    format!(" Creatures: {}", sim.creatures.len()).green(),
            //))?;
            stdout.flush().unwrap();

            if (sim.creatures.len() == 0) {
                running = false;
                dead = true
            }
        }

        ticks = ticks.wrapping_add(1);

        sleep(Duration::from_micros(10));
    }

    if dead {
        sleep(Duration::from_millis(1000));
        sim.draw_dead()
    }

    stdout.execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();

    Ok(())
}
