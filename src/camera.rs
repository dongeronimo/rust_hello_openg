extern crate cgmath;
pub struct Camera {
    fov:f32,
    width:u32,
    height:u32,
    z_near:f32,
    z_far:f32,
}
impl Camera{
    pub fn new(width:u32, height: u32) -> Camera{
        Camera{fov:45.0, 
            width:width,
            height:height,
            z_near:0.01,
            z_far:100.0}
    }
    pub fn projection_matrix(&self)->cgmath::Matrix4<f32>{
        let projection_matrix = cgmath::perspective( 
            cgmath::Rad::from(cgmath::Deg(self.fov)),   //fov, 
            self.width as f32/self.height as f32, //aspect
            self.z_near,//near z
            self.z_far); //far z;
        return projection_matrix;
    }
    pub fn view_matrix(&self)->cgmath::Matrix4<f32>{
        let camera_translation = cgmath::Vector3::new(0.0, 0.0, -30.0);
        let view_matrix = cgmath::Matrix4::from_translation(camera_translation);
        return view_matrix;
    }
}