use crate::core::area::Axis;

#[derive(Copy, Clone)]
pub struct Size {
    pub width: Constraint,
    pub height: Constraint,
}

impl Size {
    pub fn new(width: Constraint, height: Constraint) -> Size {
        Size { width, height }
    }

    pub fn get_axis(&self, axis: Axis) -> Constraint {
        match axis {
            Axis::Horizontal => self.width,
            Axis::Vertical => self.height,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Constraint {
    Maximized,
    Minimized,
    Constant(u32),
}