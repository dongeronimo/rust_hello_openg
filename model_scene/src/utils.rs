use uuid::Uuid;
use crate::transform::Transform;
pub fn create_id_and_initial_transform()->(String, Transform){
    let id = Uuid::new_v4().to_string();
    let transform = Transform::new();
    return (id, transform);
}