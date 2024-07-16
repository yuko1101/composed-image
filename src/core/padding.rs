pub struct Padding {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
}

impl Padding {
    pub fn all(pixel: u32) -> Padding {
        Padding {
            top: pixel,
            bottom: pixel,
            left: pixel,
            right: pixel,
        }
    }
    pub fn zero() -> Padding {
        Self::all(0)
    }
    pub fn top(pixel: u32) -> Padding {
        Padding {
            top: pixel,
            bottom: 0,
            left: 0,
            right: 0,
        }
    }
    pub fn bottom(pixel: u32) -> Padding {
        Padding {
            top: 0,
            bottom: pixel,
            left: 0,
            right: 0,
        }
    }
    pub fn left(pixel: u32) -> Padding {
        Padding {
            top: 0,
            bottom: 0,
            left: pixel,
            right: 0,
        }
    }
    pub fn right(pixel: u32) -> Padding {
        Padding {
            top: 0,
            bottom: 0,
            left: 0,
            right: pixel,
        }
    }
}