extern crate sdl2;//Imports sdl2
extern crate gl; //imports gl
pub fn main_loop(sdl:&sdl2::Sdl, 
    video_subsystem:&sdl2::VideoSubsystem,
    window:&sdl2::video::Window){
    //Load the function pointers.
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
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
        unsafe{
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        //TODO: more things
        window.gl_swap_window();
    }
}