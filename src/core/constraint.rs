use crate::core::area::Axis;

#[derive(Copy, Clone)]
pub struct AreaConstraint {
    pub width: Constraint,
    pub height: Constraint,
}

impl AreaConstraint {
    pub fn new(width: Constraint, height: Constraint) -> AreaConstraint {
        AreaConstraint { width, height }
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