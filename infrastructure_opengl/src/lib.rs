extern crate sdl2;
pub mod shaders;//torna a shaders publica pros usuários dessa lib.
pub mod vbo;
pub mod vao;
mod shader_utils;
mod utils;
pub fn create_gl_context(window:&sdl2::video::Window,
    video_subsystem:&sdl2::VideoSubsystem)-> sdl2::video::GLContext {
    let gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    return gl_context;
}

