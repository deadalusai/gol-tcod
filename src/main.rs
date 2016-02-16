extern crate tcod;
extern crate rand;
extern crate gol;

use tcod::console::{ Root, Console, BackgroundFlag, TextAlignment };
use tcod::system;
use tcod::input::{ Event, KeyCode };
use tcod::input;

use gol::world::World;
use gol::grid::{ Grid, Cell };
use gol::plaintext as pt;

use rand::{ thread_rng, Rng };

use std::process::{ exit };
use std::io;
use std::fs;
use std::path::{ Path };
use std::env;

fn main() {

    let (mut world, label) = 
        if let Some(ptext) = maybe_load_plaintext_from_file() {
            let world = World::new(ptext.data);
            (world, format!("{}\n{}", ptext.name, ptext.comment))
        }
        else {
            let world = create_random_world(80, 60); 
            (world, "Random".into())
        };

    let (width, height) = (world.width(), world.height());

    //glider to be written in when the user clicks
    let glider = create_glider();

    let mut root =
        Root::initializer()
            .size(width as i32, height as i32)
            .title("Game of Life")
            .init();
                    
    system::set_fps(30);

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
                    world.grid_mut().write_cells(x, y, &glider);
                }
            }
        }

        //Step the simulation
        world.step_mut();
    }
}

fn maybe_load_plaintext_from_file() -> Option<pt::PlainText> {
    
    //try and grab a filename from the first argument...
    if let Some(filename) = env::args().skip(1).next() {
    
        let path = Path::new(&filename);
        println!("Reading world from file: {}", path.display());
        
        match read_world_from_file(&path) {
            Err(e) => {
                //couldn't parse the file - bail out
                println!("Error parsing file: {}", &e);
                exit(1);
            },
            Ok(p) => Some(p)
        }
    }
    else { None }
}

fn read_world_from_file(path: &Path) -> pt::ParseResult {
    let file = try!(fs::File::open(path));
    let file = io::BufReader::new(file);
    pt::parse_plaintext(file)
}

fn create_random_world(width: usize, height: usize) -> World {
    let mut rng = thread_rng();
    let choices = [ Cell::Dead, Cell::Live ];
    let grid = Grid::from_fn(width, height, |_, _| rng.choose(&choices).unwrap().clone());
    World::new(grid)
}

fn create_glider() -> Grid {
    use gol::grid::Cell::Dead as X;
    use gol::grid::Cell::Live as O;
    Grid::from_raw(3, 3, vec![ 
        X, X, O, 
        O, X, O,
        X, O, O 
    ])
}

enum Input { Exit, Reroll, Draw(usize, usize) }

fn user_input() -> Option<Input> {
    
    let flags = input::MOUSE | input::KEY;
    match input::check_for_event(flags).map(|(_, e)| e) {
        Some(Event::Key(s)) => {
            match s.code {
                KeyCode::Escape => Some(Input::Exit),
                KeyCode::Enter => Some(Input::Reroll),
                _ => None
            }
        },
        Some(Event::Mouse(s)) if s.lbutton_pressed => {
            Some(Input::Draw(s.cx as usize, s.cy as usize))
        },
        _ => None
    }
}

fn render(w: &World, label: &str, root: &mut Root) {
    root.clear();

    for (x, y, cell) in w.grid().iter_cells() {
        if cell.is_live() {
            root.put_char(x as i32, y as i32, 'O', BackgroundFlag::Set);
        }
    }

    //Print label
    root.print_ex(0, 0, BackgroundFlag::Set, TextAlignment::Left, label);
    
    //Print generation
    root.print_ex(w.width() as i32 - 1, w.height() as i32 - 1, 
                  BackgroundFlag::Set, TextAlignment::Right, &format!("Gen: {}", w.generation()));
    
    root.flush();
}

