use gl;
use std;
use std::ffi::{CStr, CString};
extern crate cgmath;
use crate::utils::create_whitespace_cstring_with_len;
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

}

impl Program {
    //Creates a new Program using the given shaders. It returns the program if Ok or the error that happened when
    //creating the shader program if Err.
    pub fn from_shaders(shaders: &[Shader])->Result<Program, String> {
        let program_id = create_program();
        match attach_shaders_to_program(program_id, shaders){
            Err(err_msg)=> return Err(err_msg),
            Ok(_)=> return Ok(Program{id:program_id}),
        }
    }
    //The program's opengl id.
    pub fn id(&self)->gl::types::GLuint{
        return self.id;
    }
    //Set the program as the current opengl shader program.
    pub fn use_program(&self){
        unsafe{gl::UseProgram(self.id);}
    }
    //Unsets the program as the current opengl shader program.
    pub fn stop_using_program(&self){
        unsafe{gl::UseProgram(0)};
    }
    //Find the uniform shader variable with the given name or returns an error string if it can't find the uniform variable.
    //Unused uniform variables are eliminated by the opengl during shader program linkage, so if this method is returning errors
    //that is the first thing to check in the shader program.
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

    //pub fn set_mat4_uniform(&self, name:String, mat:[f32; 16]){
    pub fn set_mat4_uniform(&self, name:String, mat: cgmath::Matrix4<f32>){
        //let mat:[f32; 16] = mat.into();
        let mat: [f32; 16] = unsafe {
            let x:[[f32;4];4] = mat.into();
            std::mem::transmute(x)
        };
        let uniform_id = self.find_uniform(name).unwrap();
        unsafe{
            
            gl::UniformMatrix4fv(
                uniform_id, //location
                1, //number of matrices
                false as u8,//transpose?
                mat.as_ptr() //the matrix itself
            )
        }   
    }
}
impl Drop for Program {
    fn drop(&mut self){
        println!("dropping shader");
        unsafe{gl::DeleteProgram(self.id());}
    }
}

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
        unsafe {gl::GetShaderInfoLog(id, len,std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar,);}
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