use gl;
use std;
use std::ffi::{CStr, CString};
mod shader;
pub struct Program {
    id: gl::types::GLuint;
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]]->Result<Program, String> {
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
            return Err(error.to_string_lossy().into_owned());            
        }
        //Lida com o sucesso
        else{
            for shader in shaders {
                unsafe {gl::DetachShader(program_id, shader.id());}
            }
            Ok(Program {id: program_id});
        }
        pub fn id(&self){
            self.id;
        }
        pub fn set_used(&self){
            unsafe{gl::UseProgram(self.id);}
        }
    }
}
impl Drop for Program {
    fn drop(&mut self){
        unsafe{gl::DeleteProgram(self.id);}
    }
}