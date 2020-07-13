extern crate sdl2;
pub mod shaders;
mod shader_utils;
mod utils;
pub fn create_gl_context(window:&sdl2::video::Window,
    video_subsystem:&sdl2::VideoSubsystem)-> sdl2::video::GLContext {
    let gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    return gl_context;
}


pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}