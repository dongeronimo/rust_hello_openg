//Creates a vertex buffer object for the given array of floats.
pub fn create_vbo(vertices:&Vec<f32>)->  gl::types::GLuint {
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        bind_buffer(vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,//Target
            (vertices.len()*std::mem::size_of::<f32>())as gl::types::GLsizeiptr,//size of the buffer
            vertices.as_ptr() as *const gl::types::GLvoid,//Pointer to data
            gl::STATIC_DRAW,
        );
        unbind_buffer();
    }
    vbo
}


pub fn bind_buffer(id: gl::types::GLuint){
    unsafe{
        gl::BindBuffer(gl::ARRAY_BUFFER, id);
    }
}

pub fn unbind_buffer(){
    bind_buffer(0);
}