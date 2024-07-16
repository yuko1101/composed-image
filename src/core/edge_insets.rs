#[derive(Copy, Clone)]
pub struct EdgeInsets {
    pub top: u32,
    pub bottom: u32,
    pub left: u32,
    pub right: u32,
}

impl EdgeInsets {
    pub fn all(pixel: u32) -> EdgeInsets {
        EdgeInsets {
            top: pixel,
            bottom: pixel,
            left: pixel,
            right: pixel,
        }
    }
    pub fn zero() -> EdgeInsets {
        Self::all(0)
    }
    pub fn top(pixel: u32) -> EdgeInsets {
        EdgeInsets {
            top: pixel,
            bottom: 0,
            left: 0,
            right: 0,
        }
    }
    pub fn bottom(pixel: u32) -> EdgeInsets {
        EdgeInsets {
            top: 0,
            bottom: pixel,
            left: 0,
            right: 0,
        }
    }
    pub fn left(pixel: u32) -> EdgeInsets {
        EdgeInsets {
            top: 0,
            bottom: 0,
            left: pixel,
            right: 0,
        }
    }
    pub fn right(pixel: u32) -> EdgeInsets {
        EdgeInsets {
            top: 0,
            bottom: 0,
            left: 0,
            right: pixel,
        }
    }
}