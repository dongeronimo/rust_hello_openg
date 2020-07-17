extern crate sdl2;//Imports sdl2
extern crate gl; //imports gl
extern crate infrastructure_opengl;
use std::ffi::{CString};
pub use crate::scene_object::{SceneObject};
use model_scene::world::{World};
use model_scene::identity::Identity;
use model_scene::camera::Camera;
use model_scene::object::Object;
use cgmath::Vector3;

pub fn main_loop(sdl:&sdl2::Sdl, window:&sdl2::video::Window){
    let (window_width, window_height) = window.size();
    //Testing the scene
    let mut w = World::new(window_width, window_height);
    //Reposiciona a camera
    let cam = w.get_camera_as_mut().unwrap();
    cam.translate(Vector3::new(0.0, 0.0, -5.0));
    //Cria o objeto do triâgulo como filho de root.
    let vertices: Vec<f32> = vec![//Define os vertices
        //positions       //colors
        -0.5, -0.5, 0.0,  1.0, 0.0, 0.0,
        0.5, -0.5, 0.0,   0.0, 1.0, 0.0,
        0.0, 0.5, 0.0,    0.0, 0.0, 1.0,
    ];
    //Cria o objeto
    let test_triangle = Object::new(String::from("triangle"), //Nome do objeto
                                        w.generate_id(), //id do objeto
                                        w.get_root_as_mut().unwrap().get_id(), //id do parent
                                        vertices);//os vertices
    let root = w.get_root_as_mut();
    let root = root.unwrap();
    w.set_parent_child(root.get_id(), test_triangle.get_id());                                        
    w.add_object(test_triangle);//adiciona ao world
    

    //the event pump
    let mut event_pump = sdl.event_pump().unwrap();
    //the main loop
    'main: loop {
        for event in event_pump.poll_iter(){
            match event{
                sdl2::event::Event::Quit {..}=>break 'main,
                _ => {},
            }
        }
        set_viewport_size(&window);   
        clear_screen();
        let (vp_width, vp_height) = window.size();
        w.get_camera_as_mut().unwrap().set_viewport_dimensions(vp_width, vp_height);
        w.render_world();
        //Swap the buffers
        window.gl_swap_window();
    }
}

fn clear_screen(){
    unsafe{
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

fn set_viewport_size(window:&sdl2::video::Window){
    let (width, height) = window.size();
    unsafe {
        gl::Viewport(0,0, width as i32, height as i32);
    }
}

