extern crate sdl2;//Imports sdl2
fn main() {
    //sdl2::init must be invoked before any other sdl2 call.
    let sdl = sdl2::init()
        .unwrap();//takes the ok value or panics if it is error value.
    //initializes the video subsystem
    let video_subsystem = sdl.video().unwrap();
    let _window = video_subsystem
        .window("Game", 800,600)
        .resizable()
        .build()
        .unwrap();
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
