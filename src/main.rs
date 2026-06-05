struct Place {
    id: &str, 
    behaviour: &str,
    particle: u8, 

} 

impl Place {
    fn new(behaviour: &str) -> Place {
        Place{ id: "place", behaviour: "neutral", particle: 0 }
    }

    fn set_particle(&mut self, n: u8) {
        self.particle = n; 
    }
    
}



fn main() {
    println!("Hello, world!");
}
