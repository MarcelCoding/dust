pub trait Boundary {
    fn get_x(&self) -> u32;
    fn get_y(&self) -> u32;
    fn get_length(&self) -> u32;
    fn get_width(&self) -> u32;
}

pub trait Wall: Boundary {
    // noop
}
