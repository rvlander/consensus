
use std::vec;


#[derive(Debug)]
pub struct Gnome  {
    id: usize,
}

    
impl Gnome {
    pub fn new(id: usize) -> Self {
        Gnome{
         id
        }
    }
    
    pub fn print(&self) {
        println!("{:?}", self);
    }

    pub fn initialize(neighbours: vec::Vec<&Gnome>) {

    }

    pub fn propose_action() {

    }

    pub fn relay() {

    }

    pub fn execute_action() {

    }
}