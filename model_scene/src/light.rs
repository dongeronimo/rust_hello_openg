use crate::identity::Identity;

pub struct Light {
    name:String,
    id: u32,
    parent_id: u32,
    children: Vec<u32>,
}
impl Light{
    pub fn new (name:String,last_id: u32, parent_id:u32)->Light{
        return Light {
            name: name,
            id: last_id+1,
            parent_id: parent_id,
            children: Vec::new(),
        }
    }
}
impl Identity for Light{
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
    fn get_name(&self)->&String{
        &self.name
    }   
}


// extern crate cgmath;
// use crate::transform::Transform;
// use crate::identity::Identity;
// use crate::utils::create_id_and_initial_transform;
// pub struct Light{
//     id:String,
//     transform: Transform,
//     //TODO: Coisas especificas de luz
// }
// impl Light {
//     pub fn new() -> Light{
//         let (id, transform) = create_id_and_initial_transform();
//         return Light{
//             id: id,
//             transform: transform,
//         }
//     }
// }
// impl Identity for Light{
//     fn id(&self) -> std::string::String { self.id.clone() }
// fn parent(&self) -> std::string::String { todo!() }
// fn set_parent(&mut self, _: std::string::String) { todo!() }
// fn children(&self) -> std::vec::Vec<std::string::String> { todo!() }
// fn add_child(&mut self, _: std::string::String) { todo!() }
// fn remove_child(&mut self, _: std::string::String) { todo!() }
// }