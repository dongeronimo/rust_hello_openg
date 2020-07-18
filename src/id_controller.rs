/// Controller gerador de IDs.
pub struct IdController {
    current_id: u32
}
impl IdController {
    pub fn new()->IdController{
        println!("Created the id controller");
        return IdController{current_id:0}
    }
    pub fn generate_new(&mut self) -> u32 {
        self.current_id = self.current_id + 1;
        return self.current_id;
    }
}

#[cfg(test)]
mod tests{

    #[test]
    fn id_controller_generate_new_generates_different_numbers(){
        let mut id_controller = crate::id_controller::IdController::new();
        let v1 = id_controller.generate_new();
        let v2 = id_controller.generate_new();
        assert_ne!(v1, v2);
    }
}