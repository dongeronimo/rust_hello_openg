extern crate sdl2;//Imports sdl2
fn main() {
    //sdl2::init must be invoked before any other sdl2 call.
    let _sdl = sdl2::init()
        .unwrap();//takes the ok value or panics if it is error value.
}
