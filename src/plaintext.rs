
use gol::Grid;
use gol::Cell::*;

use std::vec::Vec;
use std::result;
use std::io;
use std::fmt;
use std::convert;

pub struct PlainText {
    pub name: String,
    pub comment: String,
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
    #[derive(PartialEq)]
    enum S { Name, Comment, Body }

    let mut state = S::Name;

    let mut name = String::new();
    let mut comment = String::new();
    let mut width: usize = 0;
    let mut rows = Vec::new();

    for line in reader.lines() {
        let line = try!(line);
        if state == S::Name {
            if !line.starts_with("!Name: ") {
                return Err(Error::NameLineMissing);
            }
            name.push_str(sub_string_from(&line, 6).unwrap_or("").trim());
            state = S::Comment;
            continue;
        }
        if state == S::Comment {
            if !line.starts_with("!") {
                state = S::Body;
            }
            else {
                if comment.len() != 0 {
                    comment.push_str("\n");
                }
                comment.push_str(sub_string_from(&line, 1).unwrap_or("").trim());
            }
        }
        if state == S::Body {
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