# starting the project
- created the project: ```$ cargo new hello-opengl```
- created the .gitignore and the readme.md.
- ran the project for the first time: ```$ cargo run```
- It works so everything is ok.

# creating a window
- Rust libraries are called crates, defined in the Cargo.toml.
- added sdl2 as a dependency.
- Build using ```$ cargo build```. It downloaded, compiled and solved the dependencies.
- To add the sdl library to the code I place ```extern crate sdl2;``` at the top of the
file.
- To test if it is at least initializing I use ```let _sdl = sdl2::init().expec("Failed to init sdl");``` at the beginning of main.