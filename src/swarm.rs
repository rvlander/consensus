use super::gnome;
use super::gnome_com;
use rand::prelude::*;
use std::vec;

#[derive(Debug)]
pub struct Swarm {
    d_max: usize,
    min_nb_gnomes: usize,
    max_nb_gnomes: usize,
    max_neighbours: usize,
    gnomes : Vec<gnome::Gnome>,
}




impl Swarm {
    pub fn new(d_max: usize, min_nb_gnomes: usize, max_nb_gnomes: usize) -> Self {
        Swarm {
            d_max, 
            min_nb_gnomes,
            max_nb_gnomes,
            max_neighbours: 5,
            gnomes: vec::Vec::new(),
        }
    }

    pub fn generate(&mut self) {
        let nb_gnomes = rand::thread_rng().gen_range(self.min_nb_gnomes..self.max_nb_gnomes);
        for x in 0..nb_gnomes {
            let g = gnome::Gnome::new(x);
            self.gnomes.push(g);
        }
    }

    pub fn run(&mut self) {
        for gnome in self.gnomes.iter_mut() {
            // let mut rng = &mut rand::thread_rng();
            // let nb_neighbours =rng.next_u32() as usize % self.max_neighbours;
            // let neighbours: vec::Vec<&gnome::Gnome> = self.gnomes.iter().filter(| &gn | gn.id != gnome.id ).choose_multiple(&mut rng, nb_neighbours);
            let com = gnome_com::GnomeCom {};
            gnome.initialize(com);
        }
    }

    pub fn print(&self) {
        println!("{:?}", self);
    }
}