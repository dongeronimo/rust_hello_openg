extern crate sdl2;
extern crate cgmath;

pub mod id_controller;
pub mod world_node;
pub mod transform;
pub mod world;
pub mod shaders;
pub mod utils;
pub mod camera;
pub mod mesh;
pub mod vao;
pub mod vbo; 

use std::rc::Rc;

use std::cell::RefCell;

use crate::world_node::WorldNode;
use crate::camera::PerspectiveCamera;
use crate::world::World;
use crate::mesh::Mesh;

fn main() {
    //SDL Window
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    let window = video_subsystem
        .window("Game", 800,600)
        .opengl()//The window will have opengl flag.
        .resizable()
        .build()
        .unwrap();
    //Opengl Context
    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    //Instantiates the world;
    let mut world = World::new();
    //Get the camera.
    let camera = world.find_by_id(world.get_camera_id()).unwrap();
    //Set the camera info in the camera object.
    let cam = Rc::new( RefCell::new( PerspectiveCamera::new(800, 600)));
    camera.borrow_mut().set_camera_info(cam);
    camera.borrow_mut().translate(cgmath::Vector3::new(0.0, 0.0, -5.0));
    camera.borrow_mut().reorient(cgmath::Vector3::new(0.0, 0.0, 1.0));
    //create a mesh and put it in a child of root
    let vertices: Vec<f32> = vec![//Define os vertices
        //positions       //colors
        -0.5, -0.5, 0.0,  1.0, 0.0, 0.0,
        0.5, -0.5, 0.0,   0.0, 1.0, 0.0,
        0.0, 0.5, 0.0,    0.0, 0.0, 1.0,
    ];
    let triangle_mesh = Rc::new(RefCell::new( Mesh::new(vertices.clone())));
    let triangle = WorldNode::to_rc(WorldNode::new(world.generate_new_id(), "triangle".to_string()));
    triangle.borrow_mut().set_mesh_info(triangle_mesh);
    triangle.borrow_mut().reorient(cgmath::Vector3::new(0.0, 0.0, 1.0));
    let root = world.find_by_id(world.get_root_id()).unwrap();
    WorldNode::set_parenhood(root.clone(), triangle.clone());
    world.add_object(triangle.clone());
    
    let triangle2_mesh = Rc::new(RefCell::new(Mesh::new(vertices.clone())));
    let triangle2 = WorldNode::to_rc(WorldNode::new(world.generate_new_id(), "triangle2".to_string()));
    triangle2.borrow_mut().set_mesh_info(triangle2_mesh);
    triangle2.borrow_mut().reorient(cgmath::Vector3::new(0.0, 0.0, 1.0));
    triangle2.borrow_mut().translate(cgmath::Vector3::new(0.0, 2.0, 0.0));
    WorldNode::set_parenhood(triangle.clone(), triangle2.clone());
    world.add_object(triangle2.clone());
    //sdl main loop
    let mut event_pump = sdl.event_pump().unwrap();
    
    'main: loop {
        for event in event_pump.poll_iter(){
            match event{
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }
        //Render scene
        //1 Clear the screen
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        //pegar a camera, atualizar o tamanho da viewport e pegar a matrix de view e projection
        let camera = world.find_by_id(world.get_camera_id()).expect("Must have a camera object at this point");
        let camera_info = camera.borrow().get_camera_info().expect("The camera must have a camera info object by now").clone();
        let (viewport_width, viewport_height) = window.size();
        camera_info.borrow_mut().set_viewport_dimensions(viewport_width, viewport_height);
        let projection_matrix = camera_info.borrow_mut().projection_matrix();
        let view_matrix = camera_info.borrow_mut().view_matrix();
        //Testando translate
        {
        let triangle = world.find_by_name("triangle".to_string()).unwrap();
        let mut triangle = triangle.borrow_mut();
        let curr_pos = triangle.get_position();
        let new_pos = curr_pos + cgmath::Vector3::new(0.00001, 0.0, 0.0);
        triangle.translate(new_pos);
        }
        //2 Render the root
        let root_id = world.get_root_id();
        let root = world.find_by_id(root_id).expect("root must be here.");
        root.borrow().render(projection_matrix, view_matrix);
        //3 Flip buffers
        window.gl_swap_window();

    }
    println!("fim");
}

