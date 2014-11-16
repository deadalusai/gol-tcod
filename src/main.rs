#![feature(if_let)]

extern crate tcod;
extern crate gol;
extern crate gol_tcod;

use tcod::{ Console, background_flag, key_code, Special, PressedOrReleased };
use gol::{ World, Dead, Live };
use gol_tcod::indexed::{ ToIndexed };
use std::rand;
use std::os;
use std::io::timer;
use std::time::Duration;

fn main() {

    let w = 80u;
    let h = 60u;
    let state = Vec::from_fn(w * h, |_| {
        if rand::random::<bool>() { Live } else { Dead }
    });

    let mut world = match World::try_create(w, h, state) {
        Ok(w) => w,
        Err(err) => {
            println!("Error creating world: {}", err);
            os::set_exit_status(1);
            return;
        }
    };
    
    let mut con = Console::init_root(world.width() as int, 
                                     world.height() as int, 
                                     "Game of Life", 
                                     false);

    loop {
        //Render world
        render(&world, &mut con);

        //Handle user input
        match user_input() {
            Pass => { },
            Exit => {
                println!("User exit");
                return;
            }
        }

        //Step the simulation
        world.step_mut();

        //Sleep a moment
        timer::sleep(Duration::milliseconds(20));
    }
}

enum UserInput {
    Pass, Exit
}

fn user_input() -> UserInput {
    if let Some(keypress) = Console::check_for_keypress(PressedOrReleased) {
        if let Special(key_code::Escape) = keypress.key {
            return Exit;
        }
    }
    else if Console::window_closed() {
        return Exit;
    }  
    Pass
}

fn render(world: &World, console: &mut Console) {
    console.clear();

    for (y, row) in world.iter_rows().indexed() {
        for (x, cell) in row.iter().indexed() {
            if cell.is_live() {
                console.put_char(x as int, y as int, '@', background_flag::Set);
            }
        }
    }

    let message = format!("Generation: {}", world.generation());
    console.print_ex(1, 1, background_flag::Set, tcod::Left, message.as_slice()); 

    Console::flush();
}