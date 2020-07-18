use std::ffi::{CString};
use crate::shaders::{Shader,Program};
use crate::vbo::create_vbo;
use crate::vao::{create_vao, draw_vao};
use cgmath::Matrix4;
use cgmath::SquareMatrix;
pub struct Mesh {
    vbo:gl::types::GLuint,
    vao:gl::types::GLuint,
    shader_program: Program,
}
impl Mesh {
    pub fn new(vertices:Vec<f32>) -> Mesh{
        let vbo = create_vbo(&vertices);
        let vao = create_vao(vbo, 3, 3, 0, 3);
        let vert_src = CString::new(include_str!("triangle.vert")).unwrap();
        let vert_shader = Shader::from_vert_source(&vert_src).unwrap();
        let frag_src = CString::new(include_str!("triangle.frag")).unwrap();
        let frag_shader = Shader::from_frag_source(&frag_src).unwrap();
        let program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
        return Mesh {
            vbo: vbo,
            vao: vao,
            shader_program: program,
        };
    }

    pub fn render(&self, perspective_matrix: Matrix4<f32>, view_matrix: Matrix4<f32>,
        model_matrix: Matrix4<f32>, parent_matrix:Matrix4<f32>){
        //Ativa o shader
        self.shader_program.use_program();
        //passa as matrizes pro shader.
        self.shader_program.set_mat4_uniform("projection_matrix".to_string(), perspective_matrix);
        self.shader_program.set_mat4_uniform("view_matrix".to_string(), view_matrix);
        self.shader_program.set_mat4_uniform("parent_model_matrix".to_string(), parent_matrix);
        self.shader_program.set_mat4_uniform("model_matrix".to_string(), model_matrix);
        //desenha o vertex array object
        draw_vao(self.vao, 0, 3);
        //desativa o shader
        self.shader_program.stop_using_program();

    }


}

impl Drop for Mesh{
    fn drop(&mut self) { 
        unsafe {
            gl::DeleteBuffers(1, &mut self.vbo);
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
    }
}