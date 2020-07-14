use crate::transform::Transform;

pub fn create_id_and_initial_transform(last_id:u32)->(u32, Transform){
    //let id = Uuid::new_v4().to_string();
    let transform = Transform::new();
    return (last_id+1, transform);
}