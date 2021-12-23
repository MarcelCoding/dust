pub trait CoordinateValue {
    // fn sqrt(&self) -> Self;
    fn as_f32(&self) -> f32;
}

impl CoordinateValue for u16 {
    fn as_f32(&self) -> f32 {
        *self as f32
    }
}

impl CoordinateValue for f32 {
    fn as_f32(&self) -> f32 {
        *self
    }
}

pub struct Position<T: CoordinateValue> {
    x: T,
    y: T,
}

impl<T: CoordinateValue> Position<T> {
    pub fn new(x: T, y: T) -> Self {
        Position { x, y }
    }

    pub fn difference(&self, other: &Self) -> f32 {
        let a = (self.x.as_f32() - other.x.as_f32()).abs();
        let b = (self.y.as_f32() - other.y.as_f32()).abs();

        a.hypot(b)
    }

    pub fn get_x(&self) -> &T {
        &self.x
    }

    pub fn get_y(&self) -> &T {
        &self.y
    }
}

#[cfg(test)]
mod test {
    use crate::position::Position;

    #[test]
    fn test_difference() {
        let pos0 = Position::new(0, 0);
        let pos1 = Position::new(3, 4);
        assert_eq!(pos0.difference(&pos1), 5.0)
    }
}
