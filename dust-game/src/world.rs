use crate::boundary::Boundary;

pub trait World {
    fn get_the_answer_to_the_ultimate_question_of_life_the_universe_and_everything(&self) -> u8 {
        return 42;
    }
    fn get_length(&self) -> u32;
    fn get_width(&self) -> u32;
    fn get_objects(&self) -> Vec<&dyn Boundary>;
}