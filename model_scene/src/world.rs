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
    //Creates a new world, with a root object and a camera. The root object name is "root"
    //and the camera name is "camera". They tend to have id 0 and id 1 respectively but that's
    //an accident of implementation and not a stone-clad feature.
    pub fn new(viewport_width:u32, viewport_height:u32) -> World{
        //Creates the root object
        let mut last_id = 0 as u32;
        let mut root = Object::new(String::from("root"), 0, 0);
        last_id = root.get_id();
        //Create the camera and set it as a child of root.
        let camera = Camera::new(String::from("camera"), last_id, root.get_id(), viewport_width, viewport_height);
        root.add_child(camera.get_id());
        let root_id = root.get_id();
        let camera_id = camera.get_id();
        let mut table: HashMap<u32, WorldObject> = HashMap::new();
        &table.insert(root.get_id(), WorldObject::Object(root));
        &table.insert(camera.get_id(), WorldObject::Camera(camera));
        return World{
            last_id: last_id,
            root_id: root_id,
            camera_id: camera_id,
            objects: table,
        }
    }
    //Generate the next id and return it. Self.last_id is set to the new id.
    pub fn generate_id(&mut self) -> u32 { 
        self.last_id = self.last_id + 1;
        self.last_id
    }
    //Get the current last id. PLEASE DON'T REUSE THE ID, THE SYSTEM PRESUMES THAT EACH
    //WORLD OBJECT HAS A DIFFERENT ID.
    pub fn get_last_id(&self) -> u32 { self.last_id }
    //Find by name, returning a ref to the world object or Err if can't find it.
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
    //Get the object by its id. Like find_by_name it returns a ref to the object if Ok
    //or a string with the error if Err.
    pub fn find_by_id(&mut self, id:u32) -> Result<&mut WorldObject, String>{
        match self.objects.get_mut(&id){
            Some(world_object)=>return Ok(world_object),
            None => return Err(format!("object not found: id={}", id)),
        };
    }
    //Get the camera if ok or gives a string with err if the find_by_id called internally
    //fails or if the WorldObject fetched by it is not a Camera.
    pub fn get_camera(&mut self) -> Result<&mut Camera, String> {
        let cam_id = self.camera_id;
        match self.find_by_id(cam_id) {
            Ok(wo)=> {
                match wo {
                    WorldObject::Camera(cam)=> return Ok(cam),
                    _ => return Err(format!("Object with id {} is not a Camera", cam_id))
                }
            },
            Err(str)=> return Err(str)
        };
    }
}