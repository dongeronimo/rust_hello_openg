extern crate cgmath;
use uuid::Uuid;
/* Position and orientation of a world object. All objects in the world must 
have this information.*/
pub struct Transform {
    id:String,
    name:String,
    //Spatial position
    position: cgmath::Vector3<f32>,
    //The direction the object is looking at
    orientation: cgmath::Vector3<f32>, 
    //the up vector
    v_up: cgmath::Vector3<f32>,
}

impl Transform {
    //Creates a new transform at (0,0,0), with orientation equals to 1,0,0 and
    //vUp = 0,1,0
    pub fn new(name:String) -> Transform {
        return Transform{ 
            name: name,
            id: Uuid::new_v4().to_string(),
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            orientation: cgmath::Vector3::new(1.0, 0.0, 0.0),
            v_up: cgmath::Vector3::new(0.0, 1.0, 0.0)
        }
    }
    pub fn translate(&mut self, translation:cgmath::Vector3<f32>){
        self.position = translation;
    }
    pub fn translation(&self)->cgmath::Vector3<f32>{
        self.position
    }
    pub fn matrix(&self)->cgmath::Matrix4<f32>{
        //Creates the identity matrix
        let identity = cgmath::Matrix4::from_cols(
            cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0), 
            cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0), 
            cgmath::Vector4::new(0.0, 0.0, 1.0, 0.0), 
            cgmath::Vector4::new(0.0, 0.0, 0.0, 1.0));
        //TODO: rotation mat
        //cgmath::Matrix4::look_at_dir(eye: Point3<S>, dir: Vector3<S>, up: Vector3<S>)
        let look_at_mat = cgmath::Matrix4::look_at_dir(cgmath::Point3::new(0.0, 0.0, 0.0),
                                    self.orientation,
                                    self.v_up);
        //Translation mat
        let translation_mat = cgmath::Matrix4::from_translation(self.translation());
        println!("translation mat = ");
        print_matrix(translation_mat);
        println!("lookat mat = ");
        print_matrix(look_at_mat);
        let result = identity * look_at_mat * translation_mat;
        println!("lookat mat = ");
        print_matrix(result);
        return result;
    }
}

pub fn print_matrix(mat:cgmath::Matrix4<f32> ){
    let x:[[f32;4];4] = mat.into();
    println!("{}, {}, {}, {}", x[0][0], x[1][0], x[2][0], x[3][0]);
    println!("{}, {}, {}, {}", x[0][1], x[1][1], x[2][1], x[3][1]);
    println!("{}, {}, {}, {}", x[0][2], x[1][2], x[2][2], x[3][2]);
    println!("{}, {}, {}, {}", x[0][3], x[1][3], x[2][3], x[3][3]);
}