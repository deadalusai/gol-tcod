
use gol::{ Grid, Cell };
use gol::Cell::*;

use std::vec::Vec;
use std::result;
use std::io;
use std::fmt;
use std::convert;
use std::str::FromStr;
use std::cmp;

pub struct PlainText {
    pub name: String,
    pub comment: String,
    pub data: Grid
}

#[derive(Debug)]
struct FillTo(usize, usize);

impl FromStr for FillTo {
    type Err = ();
    fn from_str(s: &str) -> Result<FillTo, ()> {
        let mut parts = s.split('x').map(|p| FromStr::from_str(p));
        let p1 = match parts.next() {
            None | Some(Err(..)) => return Err(()), Some(Ok(v)) => v,
        };
        let p2 = match parts.next() {
            None | Some(Err(..)) => return Err(()), Some(Ok(v)) => v,
        };
        if !parts.next().is_none() {
            return Err(())
        }
        Ok(FillTo(p1, p2))
    }
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    NameLineMissing,
    Invalid
}

pub type ParseResult = result::Result<PlainText, Error>;

impl convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            Io(ref e) => write!(fmt, "I/O Error: {}", e),
            NameLineMissing => write!(fmt, "Name line missing"),
            Invalid => write!(fmt, "Body contained invalid data"),
        }
    }
}

fn sub_string_from(source: &str, from: usize) -> Option<&str> {
    source.char_indices().nth(from).map(|(char_idx, _)| &source[char_idx..])
}

/// Parses [Plaintext](http://conwaylife.com/wiki/Plaintext) format
pub fn parse_plaintext<R>(reader: R) -> Result<PlainText, Error>
    where R: io::BufRead
{
    #[derive(PartialEq)]
    enum S { Name, Comment, Body }

    let mut state = S::Name;

    let mut name = String::new();
    let mut comment = String::new();
    let mut rows = Vec::new();
    let mut width = 0;
    let mut fill_to = FillTo (0, 0);

    for line in reader.lines() {
        let line = try!(line);
        if state == S::Name {
            if !line.starts_with("!Name:") {
                return Err(Error::NameLineMissing);
            }
            let line = sub_string_from(&line, 6).unwrap_or("").trim();
            name.push_str(line);
            state = S::Comment;
            continue;
        }
        if state == S::Comment {
            if !line.starts_with("!") {
                state = S::Body;
            }
            else if line.starts_with("!Fill to:") {
                //special fill support
                let line = sub_string_from(&line, 9).unwrap_or("").trim();
                if let Ok(fill) = FillTo::from_str(line) {
                    fill_to = fill;
                }
            }
            else {
                if comment.len() != 0 {
                    comment.push_str("\n");
                }
                let line = sub_string_from(&line, 1).unwrap_or("").trim();
                comment.push_str(line);
            }
        }
        if state == S::Body {
            let mut row = Vec::new();
            for c in line.trim().chars() {
                match c {
                    'O' => row.push(Live),
                    '.' => row.push(Dead),
                     _  => (),
                }
            }
            if rows.len() == 0 {
                width = row.len();
            }
            else if width != row.len() {
                return Err(Error::Invalid);
            }
            rows.push(row);
        }
    }

    let FillTo(fill_width, fill_height) = fill_to;

    let width = cmp::max(fill_width, width);
    let height = cmp::max(fill_height, rows.len());

    let cells = fill_and_flatten(rows, fill_to);

    Ok(PlainText {
        name: name,
        comment: comment,
        data: Grid::from_raw(width, height, cells)
    })
}
 
fn fill_and_flatten(rows: Vec<Vec<Cell>>, fill_to: FillTo) -> Vec<Cell> {
    
    let FillTo(width, height) = fill_to;

    let mut cells = Vec::with_capacity(width * height);
    
    for row in &rows {
        for cell in row {
            cells.push(cell.clone());
        }
        //Apply width padding if necessary
        if row.len() < width {
            cells.extend((0..width - row.len()).map(|_| Dead));
        }
    }
    //Apply height padding if necessary
    if rows.len() < height {
        cells.extend((0..(height - rows.len()) * width).map(|_| Dead));
    }
    

    cells
}