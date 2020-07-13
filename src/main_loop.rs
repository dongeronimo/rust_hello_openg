extern crate sdl2;//Imports sdl2
extern crate gl; //imports gl
extern crate infrastructure_opengl;
use std::ffi::{CString};
//pub use crate::shaders::Shader;
//pub use crate::shaders::Program;
pub use crate::scene_object::{SceneObject};

pub fn main_loop(sdl:&sdl2::Sdl, window:&sdl2::video::Window){
    //Creates the shader
    let vert_src = CString::new(include_str!("triangle.vert")).unwrap();
    let vert_shader = infrastructure_opengl::shaders::Shader::from_vert_source(&vert_src).unwrap();
    let frag_src = CString::new(include_str!("triangle.frag")).unwrap();
    let frag_shader = infrastructure_opengl::shaders::Shader::from_frag_source(&frag_src).unwrap();
    let mut shader_program = infrastructure_opengl::shaders::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    //the vertex data
    let vertices: Vec<f32> = vec![
        //positions       //colors
        -0.5, -0.5, 0.0,  1.0, 0.0, 0.0,
        0.5, -0.5, 0.0,   0.0, 1.0, 0.0,
        0.0, 0.5, 0.0,    0.0, 0.0, 1.0,
    ];
    //new: my scene object
    let triangle = SceneObject::create(&vertices);
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
        triangle.render(&mut shader_program, window);
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

