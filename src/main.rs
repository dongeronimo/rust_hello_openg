extern crate sdl2;//Imports sdl2
mod sdl_utils;
fn main() {
    let sdl = sdl_utils::init_sdl();
    let video_subsystem = sdl_utils::init_video_subsystem(&sdl);
    let window = sdl_utils::create_window(&video_subsystem);
    sdl_utils::main_loop(&sdl);
}
