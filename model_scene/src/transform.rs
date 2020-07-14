extern crate cgmath;
use std::fmt;

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
    pub fn translate(&mut self, pos: cgmath::Vector3<f32>){
        self.position = pos;
    }
    pub fn translation(&self)->cgmath::Vector3<f32>{
        self.position
    }
    pub fn reorient(&mut self, orientation: cgmath::Vector3<f32>){
        self.orientation = orientation;
    }
    pub fn orientation(&self)->cgmath::Vector3<f32>{
        self.orientation
    }
    pub fn change_up(&mut self, v_up: cgmath::Vector3<f32>){
        self.v_up = v_up;
    }
    pub fn v_up(&self) -> cgmath::Vector3<f32>{
        self.v_up
    }
    pub fn matrix(&self) -> cgmath::Matrix4<f32> {
        let identity = cgmath::Matrix4::from_cols(
            cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0), 
            cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0), 
            cgmath::Vector4::new(0.0, 0.0, 1.0, 0.0), 
            cgmath::Vector4::new(0.0, 0.0, 0.0, 1.0));
        let look_at_mat = cgmath::Matrix4::look_at_dir(cgmath::Point3::new(0.0, 0.0, 0.0),
            self.orientation,
            self.v_up);    
        let translation_mat = cgmath::Matrix4::from_translation(self.translation());
        let result = identity * look_at_mat * translation_mat;
        return result;
    }
}
impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x:[[f32;4];4] = self.matrix().into();
        let ln1 = format!("{}, {}, {}, {}", x[0][0], x[1][0], x[2][0], x[3][0]);
        let ln2 = format!("{}, {}, {}, {}", x[0][1], x[1][1], x[2][1], x[3][1]);
        let ln3 = format!("{}, {}, {}, {}", x[0][2], x[1][2], x[2][2], x[3][2]);
        let ln4 = format!("{}, {}, {}, {}", x[0][3], x[1][3], x[2][3], x[3][3]);
        let result = format!("{}\n{}\n{}\n{}\n", ln1, ln2, ln3, ln4);
        write!(f, "{}", result)
    }
}