use gl;
use std;
use std::ffi::{CStr};
pub use crate::utils::create_whitespace_cstring_with_len;

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

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String>{
    let id = create_shader(kind);
    match compile_shader(id, &source){
        Ok(_)=> {
            return Ok(id);
        },
        Err(msg)=>{
            return Err(msg);
        }
    }
}
/*The first step of the shader creation process is to tell opengl that you want a new shader and get the opengl id of this
newly-minted shader. This id will be used whenever you manipulate the shader.*/
fn create_shader(kind: gl::types::GLenum)->gl::types::GLuint{
    unsafe{gl::CreateShader(kind)}
}
fn compile_shader(id:gl::types::GLuint, source: &CStr)->Result<String, String>{
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 {
        let (error, len) = allocate_string_buffer_for_opengl_error(id);
        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }
        //retorna o erro
        return Err(error.to_string_lossy().into_owned())
    }
    else{
        Ok(String::from("shader compiled ok"))
    }
}

fn allocate_string_buffer_for_opengl_error(id:gl::types::GLuint)-> (std::ffi::CString,gl::types::GLint)  {
    let mut len: gl::types::GLint = 0;
    unsafe {gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);} 
    let string_buffer = create_whitespace_cstring_with_len(len as usize);
    (string_buffer, len)
}
pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader])->Result<Program, String> {
        let program_id = unsafe {gl::CreateProgram()};
        //Vincula os shaders ao programa
        for shader in shaders {
            unsafe {gl::AttachShader(program_id, shader.id())};
        }
        //Linkagem
        unsafe {gl::LinkProgram(program_id);}
        let mut success: gl::types::GLint = 1;
        unsafe{gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);}
        //Lida com erro
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);}
            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe{gl::GetProgramInfoLog(program_id, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);}
            Err(error.to_string_lossy().into_owned())
        }
        //Lida com o sucesso
        else{
            for shader in shaders {
                unsafe {gl::DetachShader(program_id, shader.id());}
            }
            Ok(Program {id: program_id})
        }
    }
    pub fn id(&self){
        self.id;
    }
    pub fn set_used(&self){
        unsafe{gl::UseProgram(self.id);}
    }
}
impl Drop for Program {
    fn drop(&mut self){
        unsafe{gl::DeleteProgram(self.id);}
    }
}