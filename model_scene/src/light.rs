extern crate cgmath;
use crate::transform::Transform;
use crate::identity::Identity;
use crate::utils::create_id_and_initial_transform;
pub struct Light{
    id:String,
    transform: Transform,
    //TODO: Coisas especificas de luz
}
impl Light {
    pub fn new() -> Light{
        let (id, transform) = create_id_and_initial_transform();
        return Light{
            id: id,
            transform: transform,
        }
    }
}
impl Identity for Light{
    fn id(&self) -> std::string::String { self.id.clone() }
}