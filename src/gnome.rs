use super::gnome_com;

#[derive(Debug)]
pub struct Gnome  {
    pub id: usize,
    pub com: Option<gnome_com::GnomeCom>,
}

    
impl Gnome {
    pub fn new(id: usize) -> Self {
        Gnome{
         id,
         com: None
        }
    }
    
    pub fn print(&self) {
        println!("{:?}", self);
    }

    pub fn initialize(&mut self, com: gnome_com::GnomeCom) {
        self.com = Some(com);
    }

    pub fn propose_action(&self) {

    }

    pub fn relay(&self) {

    }

    pub fn execute_action(&self) {

    }
}