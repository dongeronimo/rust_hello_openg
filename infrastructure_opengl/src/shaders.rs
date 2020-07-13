use gl;
use std;
use std::ffi::{CStr, CString};
pub use crate::shader_utils::{shader_from_source, create_shader,compile_shader, create_program, attach_shaders_to_program};
//Represents the shader, like the vertex shader or the fragment shader. To use the shader you load it using the from_source,
//from_vert_source or from_frag_source functions. The shader's opengl id will be in the id field and when dropped the shader
//will be deleted.
pub struct Shader {
    //the shader's opengl id.
    id: gl::types::GLuint,
}
impl Shader {
    //create the shader from the source code given. Returns a new shader if Ok or the error message as a string if Err.
    pub fn from_source(source: &CStr, kind: gl::types::GLenum)-> Result<Shader, String>{
        let id = shader_from_source(source, kind)?;
        Ok(Shader {id})
    }
    //The same thing that from_source does but it creates a vertex_shader.
    pub fn from_vert_source(source: &CStr) -> Result<Shader, String>{
        Shader::from_source(source, gl::VERTEX_SHADER)
    }
    //The same thing that from_source does but it creates a fragment shader.
    pub fn from_frag_source(source: &CStr) -> Result<Shader, String>{
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }
    //The shader's opengl id.
    pub fn id(&self) -> gl::types::GLuint{
        self.id
    }
}
impl Drop for Shader{
    //deletes the shader
    fn drop(&mut self){
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
//An opengl shader program, composed of a fragment shader and a vertex shader.
pub struct Program {
    //The program's opengl id
    id: gl::types::GLuint,
    is_in_use:bool,
}

impl Program {
    //Creates a new Program using the given shaders. It returns the program if Ok or the error that happened when
    //creating the shader program if Err.
    pub fn from_shaders(shaders: &[Shader])->Result<Program, String> {
        let program_id = create_program();
        match attach_shaders_to_program(program_id, shaders){
            Err(err_msg)=> return Err(err_msg),
            Ok(_)=> return Ok(Program{id:program_id, is_in_use:false}),
        }
    }
    //The program's opengl id.
    pub fn id(&self)->gl::types::GLuint{
        return self.id;
    }
    //Set the program as the current opengl shader program.
    pub fn set_used(&mut self){
        self.is_in_use = true;
        unsafe{gl::UseProgram(self.id);}
    }
    //Unsets the program as the current opengl shader program.
    pub fn stop_using(&mut self){
        self.is_in_use = false;
        unsafe{gl::UseProgram(0)};
    }
    //Find the uniform shader variable with the given name or returns an error string if it can't find the uniform variable.
    //Unused uniform variables are eliminated by the opengl during shader program linkage, so if this method is returning errors
    //that is the first thing to check in the shader program.
    pub fn find_uniform(&self, name:String)->Result<gl::types::GLint, String>{
        if self.is_in_use == false{
            return Err(String::from("must be used after set_used and before stop_using"));
        }
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