//Imports sdl2
extern crate sdl2;
//Imports my opengl library
extern crate infrastructure_opengl;
mod sdl_utils;
mod main_loop;
mod camera;


mod scene_object;

fn main() {
    let sdl = sdl_utils::init_sdl();
    let video_subsystem = sdl_utils::init_video_subsystem(&sdl, sdl_utils::OpenGLProfile::Core4_5);
    let window = sdl_utils::create_window(&video_subsystem);
    let _gl_context = infrastructure_opengl::create_gl_context(&window, &video_subsystem);// sdl_utils::create_gl_context(&window, &video_subsystem);
    main_loop::main_loop(&sdl, &window);
}
