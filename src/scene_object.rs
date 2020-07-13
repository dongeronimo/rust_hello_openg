extern crate gl; //imports gl
extern crate infrastructure_opengl;

pub struct SceneObject {
    vbo:gl::types::GLuint,
    vao:gl::types::GLuint,
}
impl SceneObject{
    pub fn render(&self, shader_program: &mut infrastructure_opengl::shaders::Program){
        //TODO: passar a matriz
        let foo_matrix:[f32; 16] = [1.0, 0.0, 0.0, 0.0,
                                    0.0, 1.0, 0.0, 0.0,
                                    0.0, 0.0, 1.0, 0.0,
                                    0.0, 0.0, 0.0, 1.0];
        shader_program.set_used();
        shader_program.set_mat4_uniform(String::from("foobar"), foo_matrix);     
        infrastructure_opengl::draw_vao(self.vao, 0, 3);
        shader_program.stop_using();
    }
    pub fn create(vertices:&Vec<f32>)->SceneObject{
        //creates the Vertex Buffer Object
        let vbo = infrastructure_opengl::vbo::create_vbo(vertices);
        //Creates the vertex array object
        let vao = infrastructure_opengl::vao::create_vao(vbo, 3, 3, 0, 3);
        SceneObject {vbo, vao}
    }
}
impl Drop for SceneObject{
    fn drop(&mut self){
        unsafe{
            gl::DeleteBuffers(1, &mut self.vbo);//self.vbo as *const gl::types::GLuint);
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
    }
}
