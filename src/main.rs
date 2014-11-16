#![feature(if_let)]

extern crate tcod;
extern crate gol;

use tcod::{ Console, background_flag, key_code, Special, PressedOrReleased };
use gol::{ World, Dead, Live };
use std::rand;
use std::os;
use std::io::timer;
use std::time::Duration;
use std::iter::Iterator;

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

    for (y, row) in indexed(world.iter_rows()) {
        for (x, cell) in indexed(row.iter()) {
            match *cell {
                Live => { console.put_char(x as int, y as int, '@', background_flag::Set); },
                _    => { }
            }
        }
    }

    Console::flush();
}

fn indexed<A, T: Iterator<A>>(iter: T) -> Indexed<A, T> {
    Indexed { iter: iter, idx: 0 }
}

struct Indexed<A, T> {
    iter: T,
    idx: uint
}

impl<A, T: Iterator<A>> Iterator<(uint, A)> for Indexed<A, T> {
    #[inline]
    fn next(&mut self) -> Option<(uint, A)> {
        match self.iter.next() {
            Some(v) => {
                let result = (self.idx, v);
                self.idx += 1;
                Some(result)
            },
            None => None
        }
    }
}