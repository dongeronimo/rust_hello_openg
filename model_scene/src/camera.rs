use crate::identity::Identity;
use crate::transform::Transform;

pub struct Camera {
    name:String,
    id: u32,
    parent_id: u32,
    children: Vec<u32>,
    transform: Transform,
    fov:f32,
    width:u32,
    height:u32,
    z_near:f32,
    z_far:f32,
}
impl Camera{
    pub fn new(name:String,//The name, to satisfy the Identity trait. 
               last_id:u32, //The id to satisfy the Identity trait.
               parent_id:u32, //The parent to satisfy the Identity trait
               viewport_width:u32, //Viewport width, usually is the window width
               viewport_height:u32 //Viewport height, usually is the window height.
            )->Camera{
        let transform = Transform::new();
        return Camera{
            name: name,
            id: last_id+1,
            parent_id: parent_id,
            children: Vec::new(),
            fov:45.0, 
            transform: transform,
            width:viewport_width,
            height:viewport_height,
            z_near:0.01,
            z_far:100.0,
        }
    }
    pub fn projection_matrix(&self) -> cgmath::Matrix4<f32>{
        let projection_matrix = cgmath::perspective( 
            cgmath::Rad::from(cgmath::Deg(self.fov)),   //fov, 
            self.width as f32/self.height as f32, //aspect
            self.z_near,//near z
            self.z_far); //far z;
        return projection_matrix;
    }
    pub fn view_matrix(&self) -> cgmath::Matrix4<f32>{
        let look_at_mat = cgmath::Matrix4::look_at_dir(
            cgmath::Point3::new(0.0, 0.0, 0.0), 
            self.transform.orientation(), 
            self.transform.v_up());
        let translation_mat = cgmath::Matrix4::from_translation(self.transform.translation());
        let result = translation_mat * look_at_mat;
        return result;       
    }    
    //delegating to Transform.
    pub fn translate(&mut self, pos: cgmath::Vector3<f32>){
        self.transform.translate(pos);
    }
    pub fn translation(&self)->cgmath::Vector3<f32>{
        self.transform.translation()
    }
    pub fn reorient(&mut self, orientation: cgmath::Vector3<f32>){
        self.transform.reorient(orientation);
    }
    pub fn orientation(&self)->cgmath::Vector3<f32>{
        self.transform.orientation()
    }
    pub fn change_up(&mut self, v_up: cgmath::Vector3<f32>){
        self.transform.change_up(v_up);
    }
    pub fn v_up(&self) -> cgmath::Vector3<f32>{
        self.transform.v_up()
    }
}
impl Identity for Camera{
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
    fn get_name(&self) -> &String{
        &self.name
    }
}

impl Drop for Camera {
    fn drop(&mut self) { println!("Camera {} dropped", self.get_name())}
}

// extern crate cgmath;
// use crate::transform::Transform;
// use crate::identity::Identity;


// use crate::utils::create_id_and_initial_transform;
// pub struct Camera{
//     id: u32,
//     transform: Transform,
//     fov:f32,
//     width:u32,
//     height:u32,
//     z_near:f32,
//     z_far:f32,
//     parent_id: u32,
//     children_ids: Vec<u32>,

// }
// impl Camera{
//     pub fn new(width:u32, height:u32, parent_id:String, last_id:u32) -> Camera{
//         let (id, transform) = create_id_and_initial_transform(last_id);
//         return Camera{
//             fov:45.0, 
//             width:width,
//             height:height,
//             z_near:0.01,
//             z_far:100.0,
//             id: id,
//             transform: transform,
//             parent_id: parent_id,
//             children_ids: Vec::new(),

//         }
//     }
//     pub fn projection_matrix(&self) -> cgmath::Matrix4<f32>{
//         let projection_matrix = cgmath::perspective( 
//             cgmath::Rad::from(cgmath::Deg(self.fov)),   //fov, 
//             self.width as f32/self.height as f32, //aspect
//             self.z_near,//near z
//             self.z_far); //far z;
//         return projection_matrix;
//     }
//     pub fn view_matrix(&self) -> cgmath::Matrix4<f32>{
//         let look_at_mat = cgmath::Matrix4::look_at_dir(
//             cgmath::Point3::new(0.0, 0.0, 0.0), 
//             self.transform.orientation(), 
//             self.transform.v_up());
//         let translation_mat = cgmath::Matrix4::from_translation(self.transform.translation());
//         let result = translation_mat * look_at_mat;
//         return result;       
//     }
//     pub fn set_view_dimensions(&mut self, width:u32, height:u32){
//         self.width = width;
//         self.height = height;
//     }
//     //delegating to Transform.
//     pub fn translate(&mut self, pos: cgmath::Vector3<f32>){
//         self.transform.translate(pos);
//     }
//     pub fn translation(&self)->cgmath::Vector3<f32>{
//         self.transform.translation()
//     }
//     pub fn reorient(&mut self, orientation: cgmath::Vector3<f32>){
//         self.transform.reorient(orientation);
//     }
//     pub fn orientation(&self)->cgmath::Vector3<f32>{
//         self.transform.orientation()
//     }
//     pub fn change_up(&mut self, v_up: cgmath::Vector3<f32>){
//         self.transform.change_up(v_up);
//     }
//     pub fn v_up(&self) -> cgmath::Vector3<f32>{
//         self.transform.v_up()
//     }
// }
// impl Identity for Camera {
//     fn id(&self) -> u32{ self.id }
//     fn parent(&self)->u32{
//         self.parent_id
//     }
//     fn set_parent(&mut self, parent_id:String){
//     //    self.parent_id = parent_id;
//     }

//     fn children(&self)->Vec<u32>{
//   //      return self.children();
//     }
//     fn add_child(&mut self, child_id:String){
// //        self.children().push(child_id);
//     }
//     fn remove_child(&mut self, child_id:String){
//         // let mut s = self.children().as_slice();
//         // let x = s.get(0);
//         // let y = x == Some(&child_id);
//     }
// }