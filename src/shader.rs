use gl;
use std;
use std::ffi::{CStr, CString};

pub struct Shader {
    id: gl::types::GLuint;
}
impl Shader {
    pub fn from_source(source: &CStr, kind: gl::types::GLenum)-> Result<Shader, String>{
        let id = shader_from_source(source, kind)?;
        Ok(Shader {id});
    }
    pub fn from_vert_source(source: &CStr) -> Result<Shader, String>{
        Shader::from_source(source, gl::VERTEX_SHADER);
    }
    pub fn from_frag_source(source: &CStr) -> Result<Shader, String>{
        Shader::from_source(source, gl::FRAGMENT_SHADER);
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
    let id = unsafe{gl::CreateShader(kind)};//Cria o shader no opengl
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());//Seta a string fonte do shader
        gl::CompileShader(id);//Compila o shader, algo que pode dar erro.
    }
    //Verifica se deu certo
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }
    //Deu errado.
    if success == 0{
        //Pega o tamanho da string de erro que está no mundo do opengl
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        //cria o buffer com a string cheia de whitespace.
        let error = create_whitespace_cstring_with_len(len as usize);
        //pega a informaçào de erro no shader.
        unsafe {
            gl::getShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }
        //retorna o erro
        return err(error.to_string_lossy().into_owned());
    }
    //Deu certo
    else{
        //retorna o sucesso.
        Ok(id);
    }
}

fn create_whitespace_cstring_with_len(len:usize)->CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe {CString::from_vec_unchecked(buffer)};
}