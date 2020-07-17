use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;
use crate::transform::Transform;
use cgmath::SquareMatrix;

pub type WorldNodeRefCell = RefCell<WorldNode>;
pub type WorldNodeMutableRc = Rc<WorldNodeRefCell>;
pub struct WorldNode {
    name:String,
    id: u32,
    pub parent: Option<Weak<WorldNodeRefCell>>,
    pub children: Vec<WorldNodeMutableRc>,
    transform: Transform,
}
impl WorldNode{
    ///Creates a new world node, without parent and children, with the given name
    ///and id. Ids must be unique.
    pub fn new(id:u32, name:String) -> WorldNode{
        WorldNode{
            name:name,
            id: id,
            parent: None,
            children: Vec::new(),
            transform: Transform::new(),
        }
    }
    //Consumes the given world node and return a mutable reference counter to it,
    //because I like shared pointers like the ones in c++. Many calls demand a 
    //mutable reference counter as parameter.
    pub fn to_rc(wn:WorldNode) -> WorldNodeMutableRc {
        Rc::new(RefCell::new(wn))
    }
    //Binds two world objects, one as the parent, the other as the child.
    pub fn set_parenhood(parent: WorldNodeMutableRc, child: WorldNodeMutableRc){
        let d = Rc::downgrade(&parent);
        child.borrow_mut().parent = Some(d);
        parent.borrow_mut().children.push(child);
    }
    pub fn break_parenthood(parent: WorldNodeMutableRc, child_to_remove: WorldNodeMutableRc){
        //Remove from parent's list
        let child_pos_in_parent = parent.borrow_mut().children.iter().position(|child|{
            child.borrow().get_id() == child_to_remove.borrow().get_id()
        }).expect("Child not found in parent");
        parent.borrow_mut().children.remove(child_pos_in_parent);
        //set the parent as null
        child_to_remove.borrow_mut().parent = None;
    }
    pub fn render(&self){
        println!("Rendering node name = {}, id = {}",self.name, self.id);
        let parent_model_matrix = self.get_parent_matrix_or_identity();
        //TODO: Aplicar minha model matrix.
        //TODO: Aplicar a model matrix do parent como transformaçào
        self.children.iter().for_each(|node:&WorldNodeMutableRc|{
            node.borrow().render();
        });
    }
    fn get_parent_matrix_or_identity(&self) -> cgmath::Matrix4<f32> {
        let parent_model_matrix = match &self.parent {
            Some(parent)=>{
                let p = parent.upgrade().expect("I expected parent to be alive here but someone dropped it");
                let mut p = p.borrow_mut();
                let t = p.get_transform();
                t.matrix()
            }
            None=>{
                let m : cgmath::Matrix4<f32> = cgmath::Matrix4::from_diagonal(cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0));
                m
            }
        };
        return parent_model_matrix;
    }
    pub fn get_id(&self) -> u32{ self.id }
    pub fn get_name(&self) -> &String { &self.name }
    pub fn get_transform(&mut self) ->&mut Transform { &mut self.transform}
}
impl Drop for WorldNode{
    fn drop(&mut self) { 
        println!("Dropping object {}, id {}", self.name, self.id);
    }
}

//--------------------------TESTS-----------------------------------------------
#[cfg(test)]
#[test]
fn can_create_world_node(){
    let node = WorldNode::new(0, String::from("foobar"));
    //the node is created without a parent
    let is_none = node.parent.is_none();
    assert_eq!(is_none, true);
    //the node is created without children
    let len = node.children.len();
    assert_eq!(len, 0);
    //Are the name and id correct?
    assert_eq!(node.get_name(), &String::from("foobar"));
    assert_eq!(node.get_id(), 0);
}
#[test]
fn can_convert_to_mutable_reference_counted(){
    let node = WorldNode::new(0, String::from("foobar"));
    let _node_rc = WorldNode::to_rc(node);
    //If can compile and reaches here, is ok
}
#[test]
fn can_set_parenthood(){
    //Creating as reference counter
    let parent = WorldNode::to_rc( WorldNode::new(0, String::from("able")));
    let child = WorldNode::to_rc( WorldNode::new(10, String::from("baker")));
    //linking the,
    WorldNode::set_parenhood(parent.clone(), child.clone());
    let parent_id = child.borrow()
        .parent.as_ref()
        .expect("where is the parent?")
        .upgrade()
        .expect("must have the underlying data")
        .borrow()//Borrow the reference to the returned parent
        .get_id();
    let child_id = parent.borrow() //Borrows the parent
        .children //the children vector
        .iter() //the iterator
        .find(|curr_child|{ //The closure to search for the child
           curr_child.borrow().get_id() == child.borrow().get_id()
        })
        .expect("where is the child?")//panics if child not found
        .borrow().get_id();
    assert_eq!(parent_id, parent.borrow().get_id());
    assert_eq!(child_id, child.borrow().get_id());
}
#[test]
fn can_break_parenthood(){
    let parent = WorldNode::to_rc( WorldNode::new(0, String::from("able")));
    let child = WorldNode::to_rc( WorldNode::new(10, String::from("baker")));
    WorldNode::set_parenhood(parent.clone(), child.clone());
    WorldNode::break_parenthood(parent.clone(), child.clone());
    assert_eq!(child.borrow().parent.is_none(), true);
    assert_eq!(parent.borrow().children.len(), 0);
}
#[test]
fn can_mutate_transform(){
    let obj = WorldNode::to_rc(WorldNode::new(0, String::from("foo")));
    let mut obj = obj.borrow_mut();
    let mut t:&mut Transform = obj.get_transform();
    t.translate(cgmath::Vector3::new(1.0, 2.0, 3.0));
    assert_eq!(t.translation(), cgmath::Vector3::new(1.0, 2.0, 3.0));
}