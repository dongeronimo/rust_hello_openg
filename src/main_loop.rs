extern crate sdl2;//Imports sdl2
extern crate gl; //imports gl
use std::ffi::{CString, CStr};

pub fn main_loop(sdl:&sdl2::Sdl, window:&sdl2::video::Window){
    //the event pump
    let mut event_pump = sdl.event_pump().unwrap();
    //the main loop
    'main: loop {
        for event in event_pump.poll_iter(){
            match event{
                sdl2::event::Event::Quit {..}=>break 'main,
                _ => {},
            }
        }
        set_viewport_size(&window);   
        clear_screen();
        //TODO: more things
        //Swap the buffers
        window.gl_swap_window();
    }
}

fn clear_screen(){
    unsafe{
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

fn set_viewport_size(window:&sdl2::video::Window){
    let (width, height) = window.size();
    unsafe {
        gl::Viewport(0,0, width as i32, height as i32);
    }
}

// fn shader_from_source(source: &CStr, 
//     kind: gl::types::GLuint) -> Result<gl::types::GLuint, String>{
//     let id = unsafe {gl::CreateShader(kind)};
//     let mut success: gl::types::GLint = 1;
//     unsafe {
//         gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
//         gl::CompileShader(id);
//         gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
//     }
//     if success == 0 {
//         let mut len: gl::types::GLint = 0;
//         unsafe P gl::GetProgramiv(pro)
//     }
//     Ok(id)
// }
