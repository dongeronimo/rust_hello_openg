extern crate gl; //imports gl

pub struct SceneObject {
    vbo:gl::types::GLuint,
    vao:gl::types::GLuint,
}
impl SceneObject{
    pub fn create(vertices:&Vec<f32>)->SceneObject{
        //creates the Vertex Buffer Object
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,//Target
                (vertices.len()*std::mem::size_of::<f32>())as gl::types::GLsizeiptr,//size of the buffer
                vertices.as_ptr() as *const gl::types::GLvoid,//Pointer to data
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        };
        //Creates the vertex array object
        let mut vao: gl::types::GLuint = 0;
        unsafe{
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            //Specify the data layout
            //1) Positions
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0, //index of the generic vertex attr ("layout (location=0)")
                3, //number of components per vertex attr
                gl::FLOAT, //data type
                gl::FALSE, //normalized 
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint,//BYte offset between consecutive attributes
                std::ptr::null() //offset of the first component
            );
            //2) Colors
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1, //"layout (location = 1)"
                3, //rgb
                gl::FLOAT,
                gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint,//BYte offset between consecutive attributes
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid //first color tuple is at the fourth element in the array
            );
            //unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        };
        SceneObject {vbo, vao}
    }
}
impl Drop for SceneObject{
    fn drop(&mut self){
        unsafe{
            gl::DeleteBuffers(1, self.vbo as *const gl::types::GLuint);
            gl::DeleteVertexArrays(1, self.vao as *const gl::types::GLuint);
        }
    }
}