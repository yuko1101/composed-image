#[derive(Copy, Clone)]
pub struct Size {
    pub width: Constraint,
    pub height: Constraint,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Constraint {
    Maximized,
    Minimized,
    Constant(u32),
}