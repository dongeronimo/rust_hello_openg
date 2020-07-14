use crate::identity::Identity;

pub struct Object {
    name:String,
    id: u32,
    parent_id: u32,
    children: Vec<u32>
}
impl Object{
    pub fn new(name:String,last_id:u32, parent_id:u32)->Object{
        return Object{
            name:name,
            id: last_id+1,
            parent_id: parent_id,
            children: Vec::new(),
        }
    }   
}
impl Identity for Object{
    fn get_id(&self) -> u32 { 
        self.id
    }
    fn parent(&self) -> u32 {
        self.parent_id
     }
    fn set_parent(&mut self, parent_id: u32) { 
        self.parent_id = parent_id;
     }
    fn children(&self) -> &[u32] { 
        self.children.as_slice()
    }
    fn add_child(&mut self, id:u32){
        self.children.push(id);
    }
    fn remove_child(&mut self, child_id: u32) {
        let mut iter = self.children.as_slice().iter();
        let index = iter.position(|&x| x==child_id).unwrap();
        self.children.remove(index);
    } 
    fn get_name(&self) -> &std::string::String { &self.name }
}
// extern crate cgmath;
// extern crate gl; //imports gl
// use crate::transform::Transform;
// use crate::identity::Identity;
// use crate::utils::create_id_and_initial_transform;
// use crate::camera::Camera;
// use std::ffi::{CString};
// pub struct Object {
//     id:String,
//     transform: Transform,
//     //TODO: VAO e VBO
//     is_root: bool,
//     vbo:gl::types::GLuint,
//     vao:gl::types::GLuint,
//     //TODO: Shader
//     shader_program: infrastructure_opengl::shaders::Program,
// }
// impl Object{
//     pub fn new(vertices:Vec<f32>, is_root:bool) -> Object{
//         let (id, transform) = create_id_and_initial_transform();
//         let vbo = infrastructure_opengl::vbo::create_vbo(&vertices);
//         let vao = infrastructure_opengl::vao::create_vao(vbo, 3,3,0,3);
//         let vert_src = CString::new(include_str!("triangle.vert")).unwrap();
//         let vert_shader = infrastructure_opengl::shaders::Shader::from_vert_source(&vert_src).unwrap();
//         let frag_src = CString::new(include_str!("triangle.frag")).unwrap();
//         let frag_shader = infrastructure_opengl::shaders::Shader::from_frag_source(&frag_src).unwrap();
//         let program = infrastructure_opengl::shaders::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
//         return Object{
//             id: id,
//             transform: transform,
//             vbo: vbo, 
//             vao: vao,
//             shader_program: program,
//             is_root: is_root,
//         }
//     }

//     pub fn render(&mut self, camera: &Camera){
//         if self.is_root == false{
//             let projection_matrix = camera.projection_matrix();
//             let view_matrix = camera.view_matrix();
//             let model_matrix = self.transform.matrix();
//             let mvp_matrix = projection_matrix * view_matrix * model_matrix;
//             self.shader_program.set_used();
//             self.shader_program.set_mat4_uniform(String::from("foobar"), mvp_matrix);
//             infrastructure_opengl::draw_vao(self.vao, 0, 3);
//             self.shader_program.stop_using();
//         }
//     }
// }
// impl Identity for Object{
//     fn id(&self) -> std::string::String { self.id.clone() }
// fn parent(&self) -> std::string::String { todo!() }
// fn set_parent(&mut self, _: std::string::String) { todo!() }
// fn children(&self) -> std::vec::Vec<std::string::String> { todo!() }
// fn add_child(&mut self, _: std::string::String) { todo!() }
// fn remove_child(&mut self, _: std::string::String) { todo!() }
// }
// impl Drop for Object {
//     fn drop(&mut self){
//         unsafe {
//             gl::DeleteBuffers(1, &mut self.vbo);
//             gl::DeleteVertexArrays(1, &mut self.vao);
//         }
//     }
// }