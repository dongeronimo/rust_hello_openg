extern crate cgmath;
pub struct Transform {
    position: cgmath::Vector3<f32>,
    //The direction the object is looking at
    orientation: cgmath::Vector3<f32>, 
    //the up vector
    v_up: cgmath::Vector3<f32>,
}
impl Transform {
    pub fn new()->Transform{
        return Transform{
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            orientation: cgmath::Vector3::new(1.0, 0.0, 0.0),
            v_up: cgmath::Vector3::new(0.0, 1.0, 0.0),
        }
    }
}