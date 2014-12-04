#![feature(if_let)]

extern crate tcod;
extern crate gol;

use std::rand;
use std::os;
use std::io::timer;
use std::time::Duration;

//libtcod
use tcod::{ Console, BackgroundFlag, KeyCode, Special, Printable, Pressed };

//game of life
use gol::{ World, Dead, Live };

fn main() {

    let (rows, cells) = (60, 80);
    let state = Vec::from_fn(rows * cells, |_| {
        if rand::random::<bool>() { Live } else { Dead }
    });

    let mut world = match World::try_create(rows, cells, state) {
        Ok(w) => w,
        Err(err) => {
            println!("Error creating world: {}", err);
            os::set_exit_status(1);
            return;
        }
    };
    
    let mut con = Console::init_root(world.cells() as int, 
                                     world.rows() as int, 
                                     "Game of Life", 
                                     false);

    loop {
        //Render world
        render(&world, &mut con);

        //Handle user input
        match user_input() {
            Some(UserInput::Exit) => {
                println!("User exit!");
                return;
            },
            Some(UserInput::SpawnGlider) => {
                println!("Spawning glider");
                world.write_cells(
                    //write co-ords
                    rows / 2, cells / 2,
                    //glider
                    3, 3,
                    &[ Dead, Dead, Live,
                       Live, Dead, Live,
                       Dead, Live, Live, ]
                );
            },
            None => {}
        }

        //Step the simulation
        world.step_mut();

        //Sleep a moment
        timer::sleep(Duration::milliseconds(50));
    }
}

enum UserInput {
    Exit,
    SpawnGlider
}

fn user_input() -> Option<UserInput> {
    use UserInput::{ Exit, SpawnGlider };
    
    if Console::window_closed() {
        return Some(Exit);
    }
    else if let Some(keypress) = Console::check_for_keypress(Pressed) {
        match keypress.key {
            Special(KeyCode::Escape) => return Some(Exit),
            Printable('g') => return Some(SpawnGlider),
            _ => {}
        }
    }

    None
}

fn render(world: &World, console: &mut Console) {
    console.clear();

    for (y, row) in world.iter_rows().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.is_live() {
                console.put_char(x as int, y as int, '@', BackgroundFlag::Set);
            }
        }
    }

    let message = format!("Generation: {}", world.generation());
    console.print_ex(1, 1, BackgroundFlag::Set, tcod::Left, message.as_slice()); 

    Console::flush();
}