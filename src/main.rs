extern crate sdl2;//Imports sdl2

mod sdl_utils;
mod main_loop;
fn main() {
    let sdl = sdl_utils::init_sdl();
    let video_subsystem = sdl_utils::init_video_subsystem(&sdl);
    let window = sdl_utils::create_window(&video_subsystem);
    let _gl_context = sdl_utils::create_gl_context(&window);
    main_loop::main_loop(&sdl, &video_subsystem, &window);
}
