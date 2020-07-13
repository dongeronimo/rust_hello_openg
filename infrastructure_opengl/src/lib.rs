extern crate sdl2;
extern crate cgmath;
pub mod shaders;//torna a shaders publica pros usuários dessa lib.
pub mod vbo;
pub mod vao;
pub mod transform;
mod shader_utils;
mod utils;
pub fn create_gl_context(window:&sdl2::video::Window,
    video_subsystem:&sdl2::VideoSubsystem)-> sdl2::video::GLContext {
    let gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    return gl_context;
}

pub fn draw_vao(vertex_array_object:gl::types::GLuint, first_vertex:i32, number_of_vertexes:i32){
    vao::bind_vertex_array(vertex_array_object);
    unsafe{
        gl::DrawArrays(
            gl::TRIANGLES,
            first_vertex,
            number_of_vertexes
        ); 
    }
    vao::unbind_vertex_array();
}
