extern crate cgmath;
extern crate uuid;
pub mod transform;
use std::collections::HashMap;
use uuid::Uuid;
use crate::transform::Transform;
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
pub trait Identity {
    fn id(&self)->String;
}
//------------------------------------------------------------------------------
pub struct Camera{
    id: String,
    transform: Transform,
    //TODO: Coisas especificas de camera
}
impl Camera{
    pub fn new() -> Camera{
        let id = Uuid::new_v4().to_string();
        let transform = Transform::new();
        return Camera{
            id: id,
            transform: transform,
        }
    }
}
impl Identity for Camera{
    fn id(&self) -> std::string::String { self.id.clone() }
}
//------------------------------------------------------------------------------
pub struct Light{
    id:String,
    transform: Transform,
    //TODO: Coisas especificas de luz
}
impl Light {
    pub fn new() -> Light{
        let id = Uuid::new_v4().to_string();
        let transform = Transform::new();
        return Light{
            id: id,
            transform: transform,
        }
    }
}
impl Identity for Light{
    fn id(&self) -> std::string::String { self.id.clone() }
}
//------------------------------------------------------------------------------
pub struct Object {
    id:String,
    transform: Transform,
    //TODO: Coisas espeficas de Objetos 3d
}
impl Object{
    pub fn new() -> Object{
        let id = Uuid::new_v4().to_string();
        let transform = Transform::new();
        return Object{
            id: id,
            transform: transform,
        }
    }
}
impl Identity for Object{
    fn id(&self) -> std::string::String { self.id.clone() }
}
//------------------------------------------------------------------------------
pub enum WorldObject {
    Camera(Camera),
    Light(Light),
    Object(Object),
}
struct World {
    objects: HashMap<String, WorldObject>,
    root_object_id: String,
    camera_id: String,
}
impl World{
    pub fn new() -> World{
        let root = Object::new();
        let root_id = root.id();
        let camera = Camera::new();
        let camera_id = camera.id();
        let mut map: HashMap<String, WorldObject> = HashMap::new();
        
        map.insert(root.id(), WorldObject::Object(root));
        map.insert(camera.id(), WorldObject::Camera(camera));
        return World{
            objects: map,
            root_object_id: root_id,
            camera_id: camera_id
        }
    }
}