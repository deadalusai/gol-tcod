#![feature(if_let)]

//export LIBTCOD_SRC_DIR="/home/ben/Downloads/libtcod-1.5.1/"
//cp $LIBTCOD_SRC_DIR/*.so $OUT_DIR/
//cp $LIBTCOD_SRC_DIR/terminal.png $OUT_DIR/../../../

use std::os;
use std::io::fs::PathExtensions;
use std::io::fs;

fn main() {
    
    println!("cargo:rustc-flags=-L ./lib");
}