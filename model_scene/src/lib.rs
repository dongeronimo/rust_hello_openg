extern crate cgmath;
extern crate uuid;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Transform {

}
impl Transform{
    pub fn new()->Transform{
        return Transform{}
    }
}
//------------------------------------------------------------------------------
pub struct Object {
    id:String,
    transform: Transform,
    parent_id: String,
    children: Vec<String>
}
impl Object{
    pub fn new()->Object{
        let transform = Transform::new();
        let id = Uuid::new_v4().to_string();
        return Object{
            id: id,
            transform: transform,
            parent_id: "".to_string(),
            children: Vec::new(),
        }
    }
    pub fn id(&self)->String{
        self.id.clone()
    }
}
//------------------------------------------------------------------------------
pub struct World {
    objects: HashMap<String, Object>,
    root_object_id:String,
}
impl World{
    pub fn new()->World{
        let root_object = Object::new();
        let root_object_id = root_object.id(); 
        let mut map = HashMap::new();
        map.insert(root_object.id(), root_object);
        return World{
            objects: map,
            root_object_id: root_object_id
        }
    }
}