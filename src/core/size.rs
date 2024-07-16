#[derive(Copy, Clone)]
pub enum Size {
    Maximized,
    Minimized,
    Constant(u32, u32),
}