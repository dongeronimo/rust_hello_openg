use crate::camera::Camera;
use crate::light::Light;
use crate::object::Object;
use crate::identity::Identity;
use std::collections::HashMap;

pub enum WorldObject {
    Camera(Camera),
    Light(Light),
    Object(Object),
}
pub struct World{
    last_id:u32,
    root_id:u32,
    camera_id:u32,
    objects: HashMap<u32, WorldObject>,
}
impl World{
    pub fn new() -> World{
        //Creates the root object
        let mut last_id = 0 as u32;
        let mut root = Object::new(String::from("root"), 0, 0);
        last_id = root.get_id();
        //Create the camera and set it as a child of root.
        let mut camera = Camera::new(String::from("camera"), last_id, root.get_id());
        root.add_child(camera.get_id());
        let root_id = root.get_id();
        let camera_id = camera.get_id();
        let mut table: HashMap<u32, WorldObject> = HashMap::new();
        table.insert(root.get_id(), WorldObject::Object(root));
        table.insert(camera.get_id(), WorldObject::Camera(camera));
        return World{
            last_id: last_id,
            root_id: root_id,
            camera_id: camera_id,
            objects: table,
        }
    }

    pub fn find_by_name(&self, name:String)->Result<&WorldObject, String>{
        let x = self.objects.keys().find_map(|k|{
            let curr_obj:&dyn Identity = match self.objects.get(k).unwrap(){
                WorldObject::Camera(obj)=>obj as &dyn Identity,
                WorldObject::Light(obj)=>obj as &dyn Identity,
                WorldObject::Object(obj)=>obj as &dyn Identity,
            };            
            if curr_obj.get_name().as_ref() == name {
                Some(self.objects.get(k))
            }else{
                None
            }
        });
        if x.is_none() {
            return Err(format!("Could not find object with name = {}", name));
        }else{
            let y = x.unwrap().unwrap();
            return Ok(y);
        }
    }
}