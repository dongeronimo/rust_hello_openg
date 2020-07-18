use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;
use crate::transform::Transform;
use cgmath::SquareMatrix;
use crate::camera::PerspectiveCamera;
use crate::mesh::Mesh;

pub type WorldNodeRefCell = RefCell<WorldNode>;
pub type WorldNodeMutableRc = Rc<WorldNodeRefCell>;

pub struct WorldNode {
    name:String,
    id: u32,
    pub parent: Option<Weak<WorldNodeRefCell>>,
    pub children: Vec<WorldNodeMutableRc>,
    transform: Transform,
    //Things that the node can have.
    camera: Option<Rc<RefCell<PerspectiveCamera>>>,
    mesh: Option<Rc<RefCell<Mesh>>>
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
            camera: None,
            mesh: None,
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

    pub fn render(&self, perspective_matrix: cgmath::Matrix4<f32>, view_matrix: cgmath::Matrix4<f32> ){

        //Pega a matrix do pai
        let parent_matrix:cgmath::Matrix4<f32> = match self.parent.as_ref() {
            Some(_parent) =>{
                let p = _parent.clone().upgrade().unwrap();
                let parent_matrix:cgmath::Matrix4<f32> = p.borrow().get_transform_matrix();
                parent_matrix    
            },
            None=>{
                let identity = cgmath::Matrix4::from_diagonal(cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0));
                identity
            }
        };
        //Pega a minha matrix
        let my_matrix = self.get_transform_matrix();
        //TODO: Pega o mesh info
        match self.mesh.clone() {
            Some(mi)=>{
                mi.borrow().render(perspective_matrix, view_matrix, my_matrix, parent_matrix);
            },
            _ => {}
        }
        //TODO: Manda o mesh info se renderizar.
        //Passa a renderização pros filhos
        let i = self.children.iter();
        i.for_each(|node|{

            let a = node.borrow();
            let b = a.render(perspective_matrix, view_matrix);
        });

    }

    pub fn is_root(&self)->bool { return self.parent.is_none();}
    pub fn get_id(&self) -> u32{ self.id }
    pub fn get_name(&self) -> &String { &self.name }
    pub fn get_transform(&mut self) ->&mut Transform { &mut self.transform}
    pub fn get_transform_matrix(&self)->cgmath::Matrix4<f32> {
        if self.is_root() == true {
            let identity = cgmath::Matrix4::from_diagonal(cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0));
            return identity;
        }else{
            self.transform.matrix()
        }
    }
    pub fn set_camera_info(&mut self, camera: Rc<RefCell<PerspectiveCamera>>){self.camera = Some(camera);}
    pub fn get_camera_info(&self) -> Option<Rc<RefCell<PerspectiveCamera>>> { self.camera.clone() }
    pub fn set_mesh_info(&mut self, mesh:Rc<RefCell<Mesh>>){self.mesh = Some(mesh);}
    pub fn get_mesh_info(&self) -> Option<Rc<RefCell<Mesh>>> {self.mesh.clone()}
    
    pub fn translate(&mut self, new_pos: cgmath::Vector3<f32>){
        self.transform.translate(new_pos);
        match self.camera.clone(){
            Some(cam)=>{
                let mut c = cam.borrow_mut();
                c.translate(new_pos);
            },
            _ => {}
        }
    }
    pub fn get_position(&self) -> cgmath::Vector3<f32> {self.transform.translation()}
    pub fn orientation(&self) -> cgmath::Vector3<f32> {self.transform.orientation()}
    pub fn reorient(&mut self, new_orientation: cgmath::Vector3<f32>) {
        self.transform.reorient(new_orientation);
        match self.camera.clone(){
            Some(cam)=>{
                let mut c = cam.borrow_mut();
                c.reorient(new_orientation);
            },
            _ => {}
        }
    }
}
impl Drop for WorldNode{
    fn drop(&mut self) { 
        println!("Dropping object {}, id {}", self.name, self.id);
    }
}

//--------------------------TESTS-----------------------------------------------
#[cfg(test)]
#[test]
fn can_set_camera_info(){
    let cam = Rc::new( RefCell::new( PerspectiveCamera::new(800, 600)));
    let obj = WorldNode::to_rc( WorldNode::new(100, String::from("xoxo")));
    obj.borrow_mut().set_camera_info(cam);
    assert_ne!(obj.borrow().get_camera_info().is_none(), true);
}

#[test]
fn can_manipulate_camera_info(){
    let cam = Rc::new( RefCell::new( PerspectiveCamera::new(800, 600)));
    let obj = WorldNode::to_rc( WorldNode::new(100, String::from("xoxo")));
    obj.borrow_mut().set_camera_info(cam);
    let c: Rc<RefCell<PerspectiveCamera>> = obj.borrow_mut().get_camera_info().expect("camera must be here"); 
    c.borrow_mut().translate(cgmath::Vector3::new(1.0, 2.0, 3.0));
    assert_eq!(c.borrow_mut().translation(), cgmath::Vector3::new(1.0, 2.0, 3.0));
}
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
    //It's created without a camera?
    assert_eq!(node.get_camera_info().is_none(), true);
    //is it created without a mesh?
    assert_eq!(node.get_mesh_info().is_none(), true);
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
    let t:&mut Transform = obj.get_transform();
    t.translate(cgmath::Vector3::new(1.0, 2.0, 3.0));
    assert_eq!(t.translation(), cgmath::Vector3::new(1.0, 2.0, 3.0));
}