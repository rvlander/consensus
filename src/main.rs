mod gnome;
mod swarm;
mod gnome_com;

fn main() {
    let mut s = swarm::Swarm::new(5, 20, 100);
    s.generate();
    s.run();
    s.print();
}