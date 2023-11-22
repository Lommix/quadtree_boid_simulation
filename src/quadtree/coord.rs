use std::{
    ops::{Add, Sub, Div},
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}


impl Div<i32> for Coord {
    type Output = Coord;
    fn div(self, other: i32) -> Coord {
        Coord {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;
    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_f32(x: f32, y: f32) -> Self {
        Self::new(x as i32, y as i32)
    }

    pub fn project_x(&self) -> Coord {
        Coord { x: self.x, y: 0 }
    }

    pub fn project_y(&self) -> Coord {
        Coord { x: 0, y: self.y }
    }
}
