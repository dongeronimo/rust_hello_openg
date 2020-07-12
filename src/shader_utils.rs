use gl;
use std;
use std::ffi::{CStr};
pub use crate::utils::create_whitespace_cstring_with_len;
pub use crate::shaders::Shader;

pub fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String>{
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
pub fn create_shader(kind: gl::types::GLenum)->gl::types::GLuint{
    unsafe{gl::CreateShader(kind)}
}

/*Compiles the shader. If Ok returns "shader compiled ok", if Err return the error that happened.*/
pub fn compile_shader(id:gl::types::GLuint, source: &CStr)->Result<String, String>{
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 {
        let (error, len) = allocate_string_buffer_for_opengl_error(id);
        unsafe {gl::GetShaderInfoLog(id,len,std::ptr::null_mut(),error.as_ptr() as *mut gl::types::GLchar,);}
        return Err(error.to_string_lossy().into_owned())
    }
    else{
        Ok(String::from("shader compiled ok"))
    }
}


pub fn allocate_string_buffer_for_opengl_error(id:gl::types::GLuint)-> (std::ffi::CString,gl::types::GLint)  {
    let mut len: gl::types::GLint = 0;
    unsafe {gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);} 
    let string_buffer = create_whitespace_cstring_with_len(len as usize);
    (string_buffer, len)
}

pub fn create_program()->gl::types::GLuint{
    unsafe {gl::CreateProgram()}
}

pub fn attach_shaders_to_program(program_id:gl::types::GLuint, shaders: &[Shader]) -> Result<String, String>{
    for shader in shaders {
        unsafe {gl::AttachShader(program_id, shader.id())};
    }
    unsafe {gl::LinkProgram(program_id);}
    let mut success: gl::types::GLint = 1;
    unsafe{gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);}
    if success == 0 {
        let (error, len) = allocate_string_buffer_for_opengl_error(program_id);
        unsafe{gl::GetProgramInfoLog(program_id, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);}
        Err(error.to_string_lossy().into_owned())
    }else{
        Ok(String::from("program created ok"))
    }
}