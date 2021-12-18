pub trait Boundary {
    fn get_x(&self) -> u64;
    fn get_y(&self) -> u64;
    fn get_length(&self) -> u64;
    fn get_width(&self) -> u64;
}

pub trait Wall: Boundary {
    // noop
}
