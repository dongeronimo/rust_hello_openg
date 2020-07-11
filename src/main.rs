extern crate sdl2;//Imports sdl2
fn main() {
    let sdl = init_sdl();
    let video_subsystem = init_video_subsystem(&sdl);
    let window = create_window(&video_subsystem);
    main_loop(&sdl);
}
//Initializes the sdl2 context
fn init_sdl()->sdl2::Sdl{
    //sdl2::init must be invoked before any other sdl2 call.
    let sdl = sdl2::init()
        .unwrap();//takes the ok value or panics if it is error value.
    return sdl;
}
//Initializes the video subsystem
fn init_video_subsystem(sdl:&sdl2::Sdl)->sdl2::VideoSubsystem{
    //initializes the video subsystem
    let video_subsystem = sdl.video().unwrap();   
    return video_subsystem;
}
//Creates a window with fixed size
fn create_window(video_subsystem:&sdl2::VideoSubsystem)->sdl2::video::Window{
    let _window = video_subsystem
        .window("Game", 800,600)
        .resizable()
        .build()
        .unwrap();
    return _window;
}

fn main_loop(sdl:&sdl2::Sdl){
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
        //render window content here.
    }
}