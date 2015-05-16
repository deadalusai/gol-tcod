extern crate tcod;
extern crate rand;
extern crate gol;

use tcod::console::{ Root, Console, BackgroundFlag, TextAlignment };
use tcod::input::Key::{ Special };
use tcod::input::KeyCode::{ Escape, Enter };
use tcod::input::{ KEY_PRESSED, KEY_RELEASED };

use gol::World;
use gol::Cell::{ Dead, Live };

use rand::{ random };

use std::thread;
use std::process::{ exit };

fn main() {

    let (rows, cells) = (60, 80);

    let mut root = Root::initializer()
                    .size(cells as i32, rows as i32)
                    .title("Game of Life")
                    .init();

    let mut world = create_random_world(rows, cells);

    while !root.window_closed() {
        //Render world
        render(&world, &mut root);

        //Handle user input
        if let Some(input) = user_input(&root) {
            match input {
                Input::Exit => {
                    println!("User exit");
                    exit(0);
                },
                Input::Reroll => {
                    world = create_random_world(rows, cells);
                }
            }
        }

        //Step the simulation
        world.step_mut();

        //Sleep a moment
        thread::sleep_ms(20);
    }
}

fn create_random_world(rows: usize, cells: usize) -> World {
    let state = (0..(rows * cells)).map(|_| if random::<bool>() { Live } else { Dead }).collect();

    let world = match World::try_create(rows, cells, state) {
        Ok(w) => w,
        Err(err) => {
            println!("Error creating world: {:?}", err);
            exit(1);
        }
    };

    world
}

enum Input { Exit, Reroll }

fn user_input(root: &Root) -> Option<Input> {
    if let Some(keypress) = root.check_for_keypress(KEY_PRESSED | KEY_RELEASED) {
        return match keypress.key {
            Special(Escape) => Some(Input::Exit),
            Special(Enter)  => Some(Input::Reroll),
            _______________ => None
        };
    }
    None
}

fn render(world: &World, root: &mut Root) {
    root.clear();

    for (y, row) in world.iter_rows().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.is_live() {
                root.put_char(x as i32, y as i32, '@', BackgroundFlag::Set);
            }
        }
    }

    let message = format!("Generation: {}", world.generation());
    root.print_ex(1, 1, BackgroundFlag::Set, TextAlignment::Left, &message); 

    root.flush();
}