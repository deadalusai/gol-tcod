# gol-tcod

The Game of Life rendered using libtcod.

Follow the setup instructions below and then run `cargo build`

## Setup instructions

This program depends on [tcod-rs][tcod-rs], so begin by following the instructions
there. 

By default the tcod-rs build builds libtcod from source, and so it depends on
the MinGW build chain to do so under windows. See below for my notes on working
around issues encountered.

[tcod-rs]: https://github.com/tomassedovic/tcod-rs/tree/64510dbd86388cd5667b8b92d097e722e6c28aa4


## Notes

1.  I've had difficulty getting a 64-bit MinGW build chain working. So far it has been simplest
    to use [32-bit Rust][rust-32bit] and the 32-bit MinGW tools via [mingw-get-setup.exe][mingw-get-installer]

2.  You will need the following MinGW packages (also listed on the tcod-rs instructions):

    * `mingw32-base` C compiler (gcc)
    * `mingw32-gcc-g++` C++ compiler (g++)
    * `msys-base` MSYS basic system

3.  By default the MinGW installer installs to `C:\MinGW`. Be sure to put the compiler and 
    MSYS tools directories on your `PATH`:

    * `C:\MinGW\bin`
    * `C:\MinGW\msys\1.0\bin`

4.  There [appears to be a bug][mingw-bug] in MinGW which you may see during the build in the
    form of the following error message:

        c:\mingw\include\math.h: In function 'float hypotf(float, float)':
        c:\mingw\include\math.h:635:30: error: '_hypot' was not declared in this scope

    The hacky workaround is to edit the MinGW `C:\MinGW\include\math.h` header to comment out
    the offending block of code:

        #ifndef __NO_INLINE__
        __CRT_INLINE float __cdecl hypotf (float x, float y)
        { return (float)(_hypot (x, y)); }
        #endif

    The less hacky workaround is to add `-D__NO_INLINE__` GCC compiler flag to the 
    `tcod-rs/tcod_sys/libtcod/makefiles/makefile-mingw` makefile, but I haven't found a way to
    do this without modifying the tcod-rs source.


[rust-32bit]: https://static.rust-lang.org/dist/rust-nightly-i686-unknown-linux-gnu.tar.gz
[mingw-get-installer]: https://sourceforge.net/projects/mingw/files/Installer/
[mingw-bug]: https://github.com/g-truc/glm/issues/300