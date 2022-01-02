#[derive(Debug)]
pub struct Swarm {
    d_max : usize,
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

    pub fn print(&self) {
        println!("{:?}", self)
    }
}