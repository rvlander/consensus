use super::gnome;
use rand::prelude::*;

#[derive(Debug)]
pub struct Swarm {
    d_max: usize,
    min_nb_gnomes: usize,
    max_nb_gnomes: usize,
}




impl Swarm {
    pub fn new(d_max: usize, min_nb_gnomes: usize, max_nb_gnomes: usize) -> Self {
        Swarm {
            d_max, 
            min_nb_gnomes,
            max_nb_gnomes
        }
    }

    pub fn generate(&self) {
        let nb_gnomes = rand::thread_rng().gen_range(self.min_nb_gnomes..self.max_nb_gnomes);
        for x in 0..nb_gnomes {
            let g = gnome::Gnome::new(x);
            g.print();
        }
        
    }

    pub fn print(&self) {
        println!("{:?}", self)
        
    }
}