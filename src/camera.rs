extern crate cgmath;
pub struct Camera {
    fov:f32,
    width:u32,
    height:u32,
    z_near:f32,
    z_far:f32,
    //TODO: One day I will replace these things with the Transform class
    position: cgmath::Vector3<f32>,
    direction: cgmath::Vector3<f32>,
    up: cgmath::Vector3<f32>,
}
impl Camera{
    pub fn new(width:u32, height: u32) -> Camera{
        Camera{fov:45.0, 
            width:width,
            height:height,
            z_near:0.01,
            z_far:100.0,
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            direction: cgmath::Vector3::new(0.0, 0.0, 1.0),
            up: cgmath::Vector3::new(0.0, 1.0, 0.0)}
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
        //TODO: look at
        let look_at_mat = cgmath::Matrix4::look_at_dir(
            cgmath::Point3::new(0.0, 0.0, 0.0), 
            self.direction, 
            self.up);
        //TODO: translation
        let translation_mat = cgmath::Matrix4::from_translation(self.position);
        //TODO: First look at, then translate
        let result = translation_mat * look_at_mat;
        // let camera_translation = cgmath::Vector3::new(0.0, 0.0, -30.0);
        // let view_matrix = cgmath::Matrix4::from_translation(camera_translation);
        return result;
    }
    //TODO: Use the transform class
    pub fn translate(&mut self, translation: cgmath::Vector3<f32>){
        self.position = translation;
    }
    pub fn set_direction(&mut self, direction: cgmath::Vector3<f32>){
        self.direction = direction; 
    }
    pub fn set_up(&mut self, up: cgmath::Vector3<f32>){
        self.up = up;
    }
}