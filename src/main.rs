extern crate tcod;
extern crate rand;
extern crate gol;

mod plaintext;

use tcod::console::{ Root, Console, BackgroundFlag, TextAlignment };
use tcod::input::Key::{ Special };
use tcod::input::KeyCode::{ Escape, Enter };
use tcod::input::{ Event, EventIterator };

use gol::{ World, Grid };
use gol::Cell::Dead as X;
use gol::Cell::Live as O;

use plaintext as pt;

use rand::{ thread_rng };

use std::thread;
use std::process::{ exit };
use std::io;
use std::fs;
use std::path::{ Path };

fn main() {

    let (mut world, label) = 
        if let Some(s) = std::env::args().skip(1).next() {
            println!("Reading from file: {}", s);
            let r = read_world_from_file(&Path::new(&s));
            match r {
                Err(e) => {
                    println!("Unexpected {}", e);
                    exit(1);
                },
                Ok(p) => {
                    (World::new(p.data) , p.name.unwrap_or_else(|| "unknown".to_string()))
                }
            }
        }
        else {
            (create_random_world(80, 60), "Random".to_string())
        };

    let (width, height) = (world.width(), world.height());

    //glider to be written in when the user clicks
    let glider = Grid::from_raw(3, 3, vec![ 
        X, X, O, 
        O, X, O,
        X, O, O 
    ]);

    let mut root = Root::initializer()
                    .size(width as i32, height as i32)
                    .title("Game of Life")
                    .init();

    while !root.window_closed() {
        //Render world
        render(&world, &label, &mut root);

        //Handle user input
        if let Some(input) = user_input() {
            match input {
                Input::Exit => {
                    println!("User exit");
                    exit(0);
                },
                Input::Reroll => {
                    world = create_random_world(width, height);
                },
                Input::Draw(x, y) => {
                    world.write_cells(x, y, &glider);
                }
            }
        }

        //Step the simulation
        world.step_mut();

        //Sleep a moment
        thread::sleep_ms(20);
    }
}

fn create_random_world(width: usize, height: usize) -> World {
    let mut rng = thread_rng();
    World::new(Grid::create_random(&mut rng, width, height))
}

enum Input { Exit, Reroll, Draw(usize, usize) }

fn user_input() -> Option<Input> {
    for (_, event) in EventIterator::new() {
        let input = match event {
            Event::Key(s) => {
                match s.key {
                    Special(Escape) => Some(Input::Exit),
                    Special(Enter)  => Some(Input::Reroll),
                    _______________ => None
                }
            },
            Event::Mouse(s) => {
                if s.lbutton_pressed {
                    Some(Input::Draw(s.cx as usize, s.cy as usize))
                }
                else {
                    None
                }
            }
        };

        if input.is_some() {
            return input;
        }
    }

    None
}

fn render(world: &World, label: &str, root: &mut Root) {
    root.clear();

    for (y, row) in world.iter_rows().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.is_live() {
                root.put_char(x as i32, y as i32, 'O', BackgroundFlag::Set);
            }
        }
    }

    let message = format!("{} (Generation: {})", label, world.generation());
    root.print_ex(1, 1, BackgroundFlag::Set, TextAlignment::Left, &message); 

    root.flush();
}

fn read_world_from_file(path: &Path) -> pt::ParseResult {
    let file = try!(fs::File::open(path));
    let file = io::BufReader::new(file);
    pt::parse_plaintext(file)
}

