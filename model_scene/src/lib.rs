extern crate cgmath;
extern crate uuid;
extern crate infrastructure_opengl;
pub mod identity;
pub mod transform;
pub mod camera;
pub mod light;
pub mod object;
mod utils;
use std::collections::HashMap;

use crate::identity::Identity;
use crate::camera::Camera;
use crate::light::Light;
use crate::object::Object;


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
    pub fn new(view_width: u32, view_height:u32) -> World{
        let root = Object::new(Vec::new(), true);
        let root_id = root.id();
        let camera = Camera::new(view_width, view_height);
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