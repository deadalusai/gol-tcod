
use gol::Grid;
use gol::Cell::*;

use std::vec::Vec;
use std::result;
use std::io;
use std::fmt;
use std::convert;

pub struct PlainText {
    pub name: Option<String>,
    pub comment: Option<String>,
    pub data: Grid
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

/// Parses [Plaintext](http://conwaylife.com/wiki/Plaintext) format
pub fn parse_plaintext<R>(reader: R) -> Result<PlainText, Error>
    where R: io::BufRead
{
    enum S { Name, Comment, Body }

    let mut state = S::Name;

    let mut name = None;
    let mut comment = None;
    let mut width: usize = 0;
    let mut rows = Vec::new();

    for line in reader.lines() {
        let line = try!(line);
        match state {
            S::Name => {
                if !line.starts_with("!Name: ") {
                    return Err(Error::NameLineMissing);
                }
                name = sub_string_from(&line, 6).map(|s| s.trim().to_string());
                state = S::Comment;
            },
            S::Comment => {
                if !line.starts_with("!") {
                    state = S::Body;
                }
                comment = sub_string_from(&line, 1).map(|s| s.trim().to_string());
            },
            S::Body => {
                let mut row = Vec::new();
                for c in line.trim().chars() {
                    match c {
                        'O' => row.push(Live),
                        '.' => row.push(Dead),
                        ___ => { },
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
    }

    let cells = rows.iter().flat_map(|r| r.iter().map(|c| c.clone())).collect();

    Ok(PlainText {
        name: name,
        comment: comment,
        data: Grid::from_raw(width, rows.len(), cells)
    })
}

fn sub_string_from(source: &str, from: usize) -> Option<&str> {
    source.char_indices().nth(from).map(|(char_idx, _)| &source[char_idx..])
}