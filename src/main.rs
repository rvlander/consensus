mod swarm;

fn main() {
    let s = swarm::Swarm::new(5, 20, 100);
    s.print();
}