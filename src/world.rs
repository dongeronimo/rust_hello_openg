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
    //O novo mundo tem um node?
    let root_id = w.get_root_id();
    assert_ne!(root_id, 0);
    //O novo node tem uma camera?
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

