extern crate sdl2;//Imports sdl2
extern crate gl; //imports gl
mod sdl_utils;
mod main_loop;
fn main() {
    let sdl = sdl_utils::init_sdl();
    let video_subsystem = sdl_utils::init_video_subsystem(&sdl);
    let window = sdl_utils::create_window(&video_subsystem);
    main_loop::main_loop(&sdl);
}
