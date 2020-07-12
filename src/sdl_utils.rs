extern crate sdl2;//Imports sdl2
extern crate gl; //imports gl
//Initializes the sdl2 context
pub fn init_sdl()->sdl2::Sdl{
    //sdl2::init must be invoked before any other sdl2 call.
    let sdl = sdl2::init()
        .unwrap();//takes the ok value or panics if it is error value.
    return sdl;
}
//Initializes the video subsystem
pub fn init_video_subsystem(sdl:&sdl2::Sdl)->sdl2::VideoSubsystem{
    //initializes the video subsystem
    let video_subsystem = sdl.video().unwrap();   
    return video_subsystem;
}
//Creates a window with fixed size
pub fn create_window(video_subsystem:&sdl2::VideoSubsystem)->sdl2::video::Window{
    let _window = video_subsystem
        .window("Game", 800,600)
        .opengl()//The window will have opengl flag.
        .resizable()
        .build()
        .unwrap();
    return _window;
}

pub fn create_gl_context(window:&sdl2::video::Window,
    video_subsystem:&sdl2::VideoSubsystem)-> sdl2::video::GLContext {
    let gl_context = window.gl_create_context().unwrap();
    //Load the function pointers. After that they will be available at the namespace gl
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    return gl_context;
}


