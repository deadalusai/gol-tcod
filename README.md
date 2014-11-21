# gol-tcod

The Game of Life rendered using libtcod.

Follow the setup instructions below and then run `cargo build`

## Getting started
Install SDK and libtcod for your platform.


### Linux

1. Install SDL2 through your favourite package manager.
2. Download [libtcod](http://roguecentral.org/doryen/libtcod/download/).
3. Drop `libtcod.so` into `./lib`
4. Drop `terminal.png` into `./`


### Windows

1. Download [libtcod (Visual Studio)](http://roguecentral.org/doryen/libtcod/download/).
2. Drop `libtcod-VS.dll` and `SDL.dll` into `./lib`
3. Also make a copy of `libtcod-VS.dll` called `libtcod.dll` in the `./lib` directory
4. Drop `terminal.png` into `./`
5. Put `./lib` on your `PATH`, or copy the .dlls above to the execution directory.

## Libtcod

**Note:** libtcod searches for `terminal.png` in the working directory.

**Note:** libtcod searches for `terminal.png` in the working directory.
