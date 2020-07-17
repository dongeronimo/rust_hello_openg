extern crate cgmath;
extern crate infrastructure_opengl;

pub mod world;
pub mod id_controller;
pub mod world_node;
pub mod transform;
// extern crate uuid;

// pub mod identity;
// pub mod transform;
// pub mod camera;
// pub mod light;
// pub mod object;
// pub mod world;
// pub mod render;
// mod utils;


// // use crate::identity::Identity;
// // use crate::camera::Camera;
// // use crate::light::Light;
// // use crate::object::Object;


// // //------------------------------------------------------------------------------
// // pub enum WorldObject {
// //     Camera(Camera),
// //     Light(Light),
// //     Object(Object),
// // }
// // struct World {
// //     last_id: u32,
// //     objects: HashMap<String, WorldObject>,
// //     root_object_id: String,
// //     camera_id: String,
// // }
// // impl World{
// //     pub fn new(view_width: u32, view_height:u32) -> World{
// //         let last_id = 0;
// //         let mut root = Object::new(Vec::new(), true);
// //         let root_id = root.id();
// //         let mut camera = Camera::new(view_width, view_height, root_id.clone(), );
// //         let camera_id = camera.id();
// //         let mut map: HashMap<String, WorldObject> = HashMap::new();
// //         //root as camera's parent
// //         root.add_child(camera_id.clone());
// //         //camera as root's child
// //         camera.set_parent(root_id.clone());
// //         map.insert(root.id(), WorldObject::Object(root));
// //         map.insert(camera.id(), WorldObject::Camera(camera));
// //         return World{
// //             objects: map,
// //             root_object_id: root_id,
// //             camera_id: camera_id
// //         }
// //     }
// // }