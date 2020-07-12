extern crate sdl2;//Imports sdl2

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
        .opengl()//The window 
        .resizable()
        .build()
        .unwrap();
    return _window;
}

