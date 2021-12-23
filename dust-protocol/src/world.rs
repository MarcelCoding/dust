use dust_game::boundary::Boundary;
use dust_game::world::World;
use crate::WorldMessage;

impl World for WorldMessage {


    fn get_length(&self) -> u32 {
        self.length
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_objects(&self) -> Box<&Vec<dyn Boundary>> {
        /* let mut boundaries: Vec<&dyn Boundary> = Vec::new();
        for boundary in &self.boundaries {
            boundaries.push(boundary)
        } */
        Box::new(&self.boundaries)
    }
}