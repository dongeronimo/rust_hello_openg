extern crate sdl2;//Imports sdl2
extern crate gl; //imports gl
pub fn main_loop(sdl:&sdl2::Sdl, window:&sdl2::video::Window){

    
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
        //clears the screen
        clear_screen();
        //TODO: more things
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