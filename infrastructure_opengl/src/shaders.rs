use gl;
use std;
use std::ffi::{CStr, CString};
pub use crate::shader_utils::{shader_from_source, create_shader,compile_shader, create_program, 
    attach_shaders_to_program};

pub struct Shader {
    id: gl::types::GLuint,
}
impl Shader {
    pub fn from_source(source: &CStr, kind: gl::types::GLenum)-> Result<Shader, String>{
        let id = shader_from_source(source, kind)?;
        Ok(Shader {id})
    }
    pub fn from_vert_source(source: &CStr) -> Result<Shader, String>{
        Shader::from_source(source, gl::VERTEX_SHADER)
    }
    pub fn from_frag_source(source: &CStr) -> Result<Shader, String>{
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }
    pub fn id(&self) -> gl::types::GLuint{
        self.id
    }
}
impl Drop for Shader{
    fn drop(&mut self){
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}



pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader])->Result<Program, String> {
        let program_id = create_program();
        match attach_shaders_to_program(program_id, shaders){
            Err(err_msg)=> return Err(err_msg),
            Ok(_)=> return Ok(Program{id:program_id}),
        }
    }
    pub fn id(&self)->gl::types::GLuint{
        return self.id;
    }
    pub fn set_used(&self){
        unsafe{gl::UseProgram(self.id);}
    }
    pub fn find_uniform(&self, name:String)->Result<gl::types::GLint, String>{
        let uniform_name = CString::new(name.clone()).unwrap();
        let uniform_id: gl::types::GLint;
        unsafe{
            uniform_id = gl::GetUniformLocation(self.id(), uniform_name.as_ptr());
        };
        if uniform_id == -1 {
            Err(format!("uniform not found {_name}", _name = name))
        }else{
            Ok(uniform_id)
        }
    }
}
impl Drop for Program {
    fn drop(&mut self){
        unsafe{gl::DeleteProgram(self.id());}
    }
}