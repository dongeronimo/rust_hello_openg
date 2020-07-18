use crate::transform::Transform;

pub struct PerspectiveCamera {
    transform: Transform,
    fov:f32,
    width:u32,
    height:u32,
    z_near:f32,
    z_far:f32,
}
impl PerspectiveCamera{
    pub fn new(viewport_width:u32, //Viewport width, usually is the window width
               viewport_height:u32 //Viewport height, usually is the window height.
            )->PerspectiveCamera{
        let mut transform = Transform::new();

        return PerspectiveCamera{
            fov:45.0, 
            transform: transform,
            width:viewport_width,
            height:viewport_height,
            z_near:0.01,
            z_far:100.0,
        }
    }
    pub fn set_viewport_dimensions(&mut self, w:u32, h:u32){
        self.width = w;
        self.height = h;
    }
    pub fn projection_matrix(&self) -> cgmath::Matrix4<f32>{
        let projection_matrix = cgmath::perspective( 
            cgmath::Rad::from(cgmath::Deg(self.fov)),   //fov, 
            self.width as f32/self.height as f32, //aspect
            self.z_near,//near z
            self.z_far); //far z;
        return projection_matrix;
    }
    pub fn view_matrix(&self) -> cgmath::Matrix4<f32>{
        let look_at_mat = cgmath::Matrix4::look_at_dir(
            cgmath::Point3::new(0.0, 0.0, 0.0), 
            self.transform.orientation(), 
            self.transform.v_up());
        let translation_mat = cgmath::Matrix4::from_translation(self.transform.translation());
        let result = translation_mat * look_at_mat;
        return result;       
    }    
    //delegating to Transform.
    pub fn translate(&mut self, pos: cgmath::Vector3<f32>){
        self.transform.translate(pos);
    }
    pub fn translation(&self)->cgmath::Vector3<f32>{
        self.transform.translation()
    }
    pub fn reorient(&mut self, orientation: cgmath::Vector3<f32>){
        self.transform.reorient(orientation);
    }
    pub fn orientation(&self)->cgmath::Vector3<f32>{
        self.transform.orientation()
    }
    pub fn change_up(&mut self, v_up: cgmath::Vector3<f32>){
        self.transform.change_up(v_up);
    }
    pub fn v_up(&self) -> cgmath::Vector3<f32>{
        self.transform.v_up()
    }
}


