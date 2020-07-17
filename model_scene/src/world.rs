use crate::world_node::{WorldNode, WorldNodeMutableRc};
use crate::id_controller::IdController;
use std::collections::HashMap;
///A world containing World Objects organized in a tree hiararchy, with it's initial
///element being the root object.
///All World Objects must have different Ids.
///Cycles will cause the program to hang in an infinte loop and crash with stack overflow
///due to the recursion. 
pub struct World {
    root_object_id: u32,
    camera_id: u32,
    id_controller: IdController,
    objects: HashMap<u32, WorldNodeMutableRc>,
}
impl World {
    ///Create a new world with a root object and a camera as child of the root object.
    ///The camera and the root can be obtained using get_camera_id and get_root_id respectively.
    pub fn new() -> World{
        let mut id_controller = IdController::new();
        let root = WorldNode::to_rc(WorldNode::new(id_controller.generate_new(), 
            "ROOT".to_string()));
        let root_id = root.borrow().get_id();
        let camera = WorldNode::to_rc(WorldNode::new(id_controller.generate_new(),
            "CAMERA".to_string()));
        let camera_id = camera.borrow().get_id();
        WorldNode::set_parenhood(root.clone(), camera.clone());
        let mut objects : HashMap<u32, WorldNodeMutableRc> = HashMap::new();
        objects.insert(root_id, root.clone());
        objects.insert(camera_id, camera.clone());
        return World {
            root_object_id: root_id,
            camera_id: camera_id,
            id_controller: id_controller,
            objects: objects,
        };
    }
    ///Return the id of the root object.
    pub fn get_root_id(&self) -> u32 { self.root_object_id}
    ///Return the id of the camera.
    pub fn get_camera_id(&self) -> u32 {self.camera_id}
    ///Generate a new id. Use this when instantiating new WorldObjects.
    pub fn generate_new_id(&mut self) -> u32 { self.id_controller.generate_new()}
    ///Searches for an object by name.
    pub fn find_by_name(&self, name:String) -> Option<WorldNodeMutableRc>{
        let object_id = self.objects.keys().find(|k|{
            let curr_node = self.objects.get(k).expect("if the element's key is in the key list it is expected that i would be able to find it here");
            if curr_node.borrow().get_name() == &name {
                true
            }else{
                false
            }
        });
        match object_id {
            Some(id)=>{
                let obj = self.objects.get(id).unwrap();
                return Some(obj.clone());
            },
            None=>return None,
        };
    }
    ///Searches for an object by id.
    pub fn find_by_id(&self, id:u32) -> Option<WorldNodeMutableRc> {
        let o = self.objects.get(&id);
        if o.is_none() {
            None
        }else{
            Some(o.unwrap().clone())
        }
    }

    pub fn add_object(&mut self, obj:WorldNodeMutableRc){
        self.objects.insert(obj.borrow().get_id(), obj.clone());
    }
    pub fn remove_object(&mut self, id:u32){
        self.objects.remove(&id);
    }
}
impl Drop for World {
    fn drop(&mut self) { 
        println!("Dropping world");
    }
}

//--------------------------TESTS-----------------------------------------------
#[cfg(test)]
#[test]
fn can_create_world(){
    let w = World::new();
    //TODO: O novo mundo tem um node?
    let root_id = w.get_root_id();
    assert_ne!(root_id, 0);
    //TODO: O novo node tem uma camera?
    let camera_id = w.get_camera_id();
    assert_ne!(camera_id, 0);
}
#[test]
fn can_get_new_ids(){
    let mut w = World::new();
    let id0 = w.generate_new_id();
    let id1 = w.generate_new_id();
    assert_ne!(id0, id1);
}
#[test]
fn can_find_object(){
    let w = World::new();
    let root = w.find_by_name("ROOT".to_string());
    assert_ne!(root.is_none(), true);
    assert_eq!(root.unwrap().borrow().get_name(), &"ROOT".to_string());
    let root = w.find_by_id(1);
    assert_ne!(root.is_none(), true);
    assert_eq!(root.unwrap().borrow().get_name(), &"ROOT".to_string());
}
#[test]
fn can_add_object(){
    let mut w = World::new();
    let root = w.find_by_name("ROOT".to_string()).unwrap();
    let new_obj = WorldNode::to_rc(WorldNode::new(w.generate_new_id(), "foobar".to_string()));
    WorldNode::set_parenhood(root.clone(), new_obj.clone());
    w.add_object(new_obj);
    let found_obj = w.find_by_name("foobar".to_string());
    assert_ne!(found_obj.is_none(), true);
}
#[test]
fn can_remove_object(){
    let mut w = World::new();
    let root = w.find_by_name("ROOT".to_string()).unwrap();
    let new_obj = WorldNode::to_rc(WorldNode::new(w.generate_new_id(), "foobar".to_string()));
    WorldNode::set_parenhood(root.clone(), new_obj.clone());
    w.add_object(new_obj.clone());
    w.remove_object(new_obj.borrow().get_id());
    let found_obj = w.find_by_id(new_obj.borrow().get_id());
    assert_eq!(found_obj.is_none(), true);

}




// use crate::camera::Camera;
// use crate::light::Light;
// use crate::object::Object;
// use crate::identity::Identity;
// use std::collections::HashMap;
// use crate::render::Render;

// pub enum WorldObject {
//     Camera(Camera),
//     Light(Light),
//     Object(Object),
// }
// pub struct World{
//     last_id:u32,
//     root_id:u32,
//     camera_id:u32,
//     objects: HashMap<u32, WorldObject>,
// }
// impl World{
//     //Creates a new world, with a root object and a camera. The root object name is "root"
//     //and the camera name is "camera". They tend to have id 0 and id 1 respectively but that's
//     //an accident of implementation and not a stone-clad feature.
//     pub fn new(viewport_width:u32, viewport_height:u32) -> World{
//         //Creates the root object
//         let mut last_id = 0 as u32;
//         let mut root = Object::new(String::from("root"), last_id, 0, Vec::new());
//         last_id = root.get_id();
//         //Create the camera and set it as a child of root.
//         let camera = Camera::new(String::from("camera"), last_id+1, root.get_id(), viewport_width, viewport_height);
//         last_id = camera.get_id();
//         root.add_child(camera.get_id());
//         let root_id = root.get_id();
//         let camera_id = camera.get_id();
//         let mut table: HashMap<u32, WorldObject> = HashMap::new();
//         table.insert(root.get_id(), WorldObject::Object(root));
//         table.insert(camera.get_id(), WorldObject::Camera(camera));
//         return World{
//             last_id: last_id,
//             root_id: root_id,
//             camera_id: camera_id,
//             objects: table,
//         }
//     }
//     //Generate the next id and return it. Self.last_id is set to the new id.
//     pub fn generate_id(&mut self) -> u32 { 
//         let new_id = self.last_id + 1;
//         self.last_id = new_id;
//         self.last_id
//     }
//     //Get the current last id. PLEASE DON'T REUSE THE ID, THE SYSTEM PRESUMES THAT EACH
//     //WORLD OBJECT HAS A DIFFERENT ID.
//     pub fn get_last_id(&self) -> u32 { self.last_id }
//     //Find by name, returning a ref to the world object or Err if can't find it.
//     pub fn find_by_name(&self, name:String)->Result<&WorldObject, String>{
//         let x = self.objects.keys().find_map(|k|{
//             let curr_obj:&dyn Identity = match self.objects.get(k).unwrap(){
//                 WorldObject::Camera(obj)=>obj as &dyn Identity,
//                 WorldObject::Light(obj)=>obj as &dyn Identity,
//                 WorldObject::Object(obj)=>obj as &dyn Identity,
//             };            
//             if curr_obj.get_name().as_ref() == name {
//                 Some(self.objects.get(k))
//             }else{
//                 None
//             }
//         });
//         if x.is_none() {
//             return Err(format!("Could not find object with name = {}", name));
//         }else{
//             let y = x.unwrap().unwrap();
//             return Ok(y);
//         }
//     }
//     //Get the object by its id. Like find_by_name it returns a ref to the object if Ok
//     //or a string with the error if Err.
//     pub fn find_by_id(&mut self, id:u32) -> Result<&mut WorldObject, String>{
//         match self.objects.get_mut(&id){
//             Some(world_object)=>return Ok(world_object),
//             None => return Err(format!("object not found: id={}", id)),
//         };
//     }
//     //Get the camera if ok or gives a string with err if the find_by_id called internally
//     //fails or if the WorldObject fetched by it is not a Camera.
//     pub fn get_camera_as_mut(&mut self) -> Result<&mut Camera, String> {
//         let cam_id = self.camera_id;
//         match self.find_by_id(cam_id) {
//             Ok(wo)=> {
//                 match wo {
//                     WorldObject::Camera(cam)=> return Ok(cam),
//                     _ => return Err(format!("Object with id {} is not a Camera", cam_id))
//                 }
//             },
//             Err(str)=> return Err(str)
//         };
//     }

//     pub fn get_camera(&self) -> Result<&Camera, String>{
//         let cam_id = self.camera_id;
//         match self.objects.get(&cam_id) {
//             Some(wo)=>{
//                 match wo{
//                     WorldObject::Camera(cam)=> return Ok(cam),
//                     _ => return Err(format!("Object {} is not a camera.",cam_id))
//                 }
//             },
//             None => return Err(format!("Object {} does not exist", cam_id))
//         }
//     }

//     pub fn get_root_as_mut(&mut self) -> Result<&mut Object, String> {
//         let root_id = self.root_id;
//         match self.find_by_id(root_id){
//             Ok(wo)=>{
//                 match wo {
//                     WorldObject::Object(obj) => return Ok(obj),
//                     _ => return Err(format!("Object with id {} is not an object", root_id))
//                 }
//             },
//             Err(str)=>return Err(str)
//         };
//     }

//     pub fn get_root(&self)->Result<&Object, String>{
//         match self.find_world_object_by_id(self.root_id).unwrap(){
//             WorldObject::Object(obj)=>return Ok(obj),
//             _ => return Err(format!("is not Object"))
//         }
//     }

//     pub fn set_parent_child(&mut self, parent_id:u32, child_id: u32){
//         let parent = self.find_by_id(parent_id).unwrap();
//         match parent {
//             WorldObject::Camera(cam)=> cam.add_child(child_id),
//             WorldObject::Light(light)=> light.add_child(child_id),
//             WorldObject::Object(obj)=> obj.add_child(child_id),
//         };
//     }

//     pub fn add_object(&mut self, obj:Object){
//         let parent = 
//         let l0 = self.objects.len();
//         self.objects.insert(obj.get_id(), WorldObject::Object(obj));
//         println!("Number of World Objects increased from {} to {}.", l0, self.objects.len());
//     }

//     pub fn find_world_object_by_id(&self, id:u32) -> Result<&WorldObject, String>{
//         match self.objects.get(&id) {
//             Some(wo)=>{
//                 return Ok(wo);
//             },
//             None => return Err(format!("id {} not found",self.root_id))
//         }
//     }

//     pub fn render_world(&self) {
//         println!("Beginning renderization");
//         //TODO: Pegar a matriz view e projection da camera
//         let camera = self.get_camera().unwrap();
//         let projection_matrix = camera.projection_matrix();
//         let view_matrix = camera.view_matrix();
//         //TODO: Mandar renderizar o root.
//         let root = self.get_root().unwrap();
//         let identity_mat = cgmath::Matrix4::new(1.0, 0.0, 0.0, 0.0, 
//                                                 0.0, 1.0, 0.0, 0.0, 
//                                                 0.0, 0.0, 1.0, 0.0, 
//                                                 0.0, 0.0, 0.0, 1.0);
//         root.render(projection_matrix, view_matrix, identity_mat, self);
//     }
// }