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
        match rand::random::<bool>() { true => Live, false => Dead }
    });

    let mut world = match World::try_create(w, h, state) {
        Ok(w) => w,
        Err(err) => {
            println!("Error creating world: {}", err);
            os::set_exit_status(1);
            return;
        }
    };
    
    let mut con = Console::init_root(80, 50, "Game of Life", false);

    loop {
        //Render world
        render(&world, &mut con);

        //Step the simulation
        world.step_mut();

        if let Some(keypress) = Console::check_for_keypress(PressedOrReleased) {
            match keypress.key {
                Special(key_code::Escape) => {
                    println!("Exiting");
                    return;
                },
                _ => { }
            }
        }

        //Sleep a moment
        timer::sleep(Duration::milliseconds(20));
    }
}

fn render(world: &World, console: &mut Console) {
    console.clear();

    for (y, row) in world.iter_rows().indexed() {
        for (x, cell) in row.iter().indexed() {
            match *cell {
                Live => { console.put_char(x as int, y as int, '@', background_flag::Set); },
                _    => { }
            }
        }
    }

    let message = format!("Generation: {}", world.generation());
    console.print_ex(1, 1, background_flag::Set, tcod::Left, message.as_slice()); 

    Console::flush();
}