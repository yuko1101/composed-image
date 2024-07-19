#[derive(Copy, Clone)]
pub struct Size {
    pub width: Constraint,
    pub height: Constraint,
}

#[derive(Copy, Clone)]
pub enum Constraint {
    Maximized,
    Minimized,
    Constant(u32),
}