# starting the project
1. created the project: ```$ cargo new hello-opengl```
2. created the .gitignore and the readme.md.
3. ran the project for the first time: ```$ cargo run```
4. It works so everything is ok.

# creating a window
1. Rust libraries are called crates, defined in the Cargo.toml.
2. added sdl2 as a dependency.
3. Build using ```$ cargo build```. It downloaded, compiled and solved the dependencies.
4. To add the sdl library to the code I place ```extern crate sdl2;``` at the top of the
file.
5. To test if it is at least initializing I use ```let _sdl = sdl2::init().unwrap();``` at the beginning of main.
6. sdl has a video subsystem that must be initialized to create a window. It is initialized using ```let video_subsystem = sdl.video().unwrap();```.
7. To create the actual window I use:
```
    let _window = video_subsystem
        .window("Game", 800,600)
        .resizable()
        .build()
        .unwrap();
```
8. At this moment the window will either flash on the screen and be destroyed when sdl goes out of scope at the end of main or if I put a loop after the creation will show an unresponsive window and the process will have to be killed by the OS using something like xkill.
9. To handle events I get SDL's event pump after creating the window
```
let mut event_pump = sdl.event_pump().unwrap();
```
10. The main loop, using the event pump will look like this:
```
    'main: loop {
        for event in event_pump.poll_iter(){
            match event{
                sdl2::event::Event::Quit {..}=>break 'main,
                _ => {},
            }
        }
        //render window content here.
    }
```
- ```'main: loop``` gives a name to the outer loop.
- inside the main loop I iterate the event pump events and process them. The only one I am dealing with right now is the quit event. All the others are ignored using ```_ => {}``` .
- In the quit event I break the name outer loop using the label I have given. That ends the main app loop and lets the main function proceed to it`s end. When the sdl variable leaves the scope the sdl system will be destroyed.
- So I can successfully show a window using sdl and rust.

# bibliography
- https://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-01-window.html