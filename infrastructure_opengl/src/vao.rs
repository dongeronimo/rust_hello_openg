pub use crate::vbo::{bind_buffer, unbind_buffer};
//Create a vertex array object. The vertex positions will be placed at "layout (location=0)", the colors at "layout (location=1)".
// position_size is the number of floats that represents the vertex position. If xyx then it will be 3. Colour size is the number
// of floats that represent the colour. If colour is rgb then it's size would be 3. 
// position_stride is where the position data begins in a vertex tuple and colour_stride is where the colour information starts in 
// a tuple. For example if the tuple is xyzrgb position_stride would be 0 and colour_stride would be 3.
pub fn create_vao(vbo: gl::types::GLuint, 
                  position_size: i32, 
                  colour_size:i32, 
                  position_stride:i32, 
                  colour_stride:i32)-> gl::types::GLuint  {
    let mut vao: gl::types::GLuint = 0;
    let vertex_tuple_size = position_size + colour_size;
    let vertex_tuple_size = vertex_tuple_size as usize;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        bind_vertex_array(vao);
        bind_buffer(vbo);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0, //index of the generic vertex attr ("layout (location=0)")
            position_size, //number of components per vertex attr
            gl::FLOAT, //data type
            gl::FALSE, //normalized 
            (vertex_tuple_size * std::mem::size_of::<f32>()) as gl::types::GLint,//BYte offset between consecutive attributes
            (position_stride as usize * std::mem::size_of::<f32>()) as *const gl::types::GLvoid 
        );
        //2) Colors
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1, //"layout (location = 1)"
            colour_size, //rgb
            gl::FLOAT,
            gl::FALSE,
            (vertex_tuple_size * std::mem::size_of::<f32>()) as gl::types::GLint,//BYte offset between consecutive attributes
            (colour_stride as usize * std::mem::size_of::<f32>()) as *const gl::types::GLvoid //first color tuple is at the fourth element in the array
        );
        //unbinds the buffers at the end of the process.
        unbind_buffer();
        unbind_vertex_array();
    }
    vao
}

pub fn bind_vertex_array(id: gl::types::GLuint){
    unsafe {
        gl::BindVertexArray(id);
    }
}

pub fn unbind_vertex_array(){
    bind_vertex_array(0);
}