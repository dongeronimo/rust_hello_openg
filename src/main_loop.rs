extern crate sdl2;//Imports sdl2
extern crate gl; //imports gl
use std::ffi::{CString, CStr};
pub use crate::shaders::Shader;
pub use crate::shaders::Program;

pub fn main_loop(sdl:&sdl2::Sdl, window:&sdl2::video::Window){
    //Creates the shader
    let vert_src = CString::new(include_str!("triangle.vert")).unwrap();
    let vert_shader = Shader::from_vert_source(&vert_src).unwrap();
    let frag_src = CString::new(include_str!("triangle.frag")).unwrap();
    let frag_shader = Shader::from_frag_source(&frag_src).unwrap();
    let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    //the vertex data
    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];
    //Creates the vertex buffer object
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,//Target
            (vertices.len()*std::mem::size_of::<f32>())as gl::types::GLsizeiptr,//size of the buffer
            vertices.as_ptr() as *const gl::types::GLvoid,//Pointer to data
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    let mut vao: gl::types::GLuint = 0;
    unsafe{
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        //Specify the data layout
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0, //index of the generic vertex attr ("layout (location=0)")
            3, //number of components per vertex attr
            gl::FLOAT, //data type
            gl::FALSE, //normalized 
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint,//BYte offset between consecutive attributes
            std::ptr::null() //offset of the first component
        );
        //unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

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
        //activates the shader program
        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                3
            )
        }
        

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

