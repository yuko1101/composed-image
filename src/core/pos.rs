use std::ops::Sub;

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }

    pub fn offset(&mut self, offset: Pos) {
        self.x += offset.x;
        self.y += offset.y;
    }

    pub fn offset_axis(&mut self, axis: Axis, offset: i32) {
        match axis {
            Axis::Horizontal => self.x += offset,
            Axis::Vertical => self.y += offset,
        }
    }

    pub fn into_array(self) -> [i32; 2] {
        [self.x, self.y]
    }

    pub fn from_array(array: [i32; 2]) -> Pos {
        Pos {
            x: array[0],
            y: array[1],
        }
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, other: Pos) -> Pos {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

macro_rules! pos {
    ($x:expr, $y:expr) => {
        crate::core::pos::Pos::new($x, $y)
    };
}

pub(crate) use pos;
use crate::core::area::Axis;