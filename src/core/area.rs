#[derive(Debug, Clone, Copy)]
pub struct Area {
    pub width: u32,
    pub height: u32,
}

impl Area {
    pub fn new(width: u32, height: u32) -> Area {
        Area { width, height }
    }

    pub fn into_array(self) -> [u32; 2] {
        [self.width, self.height]
    }

    pub fn from_array(array: [u32; 2]) -> Area {
        Area {
            width: array[0],
            height: array[1],
        }
    }

    pub fn into_option(self) -> OptionArea {
        OptionArea {
            width: Some(self.width),
            height: Some(self.height),
        }
    }

    pub fn single_axis(self, axis: Axis) -> SingleAxisArea {
        match axis {
            Axis::Horizontal => SingleAxisArea {
                main_axis: self.width,
                axis,
            },
            Axis::Vertical => SingleAxisArea {
                main_axis: self.height,
                axis,
            },
        }
    }

    pub fn axis(&self, axis: Axis) -> u32 {
        match axis {
            Axis::Horizontal => self.width,
            Axis::Vertical => self.height,
        }
    }

    pub fn axis_mut(&mut self, axis: Axis) -> &mut u32 {
        match axis {
            Axis::Horizontal => &mut self.width,
            Axis::Vertical => &mut self.height,
        }
    }


}

macro_rules! area {
    ($width:expr, $height:expr) => {
        crate::core::area::Area::new($width, $height)
    };
}

pub(crate) use area;

#[derive(Clone, Copy, Debug)]
pub struct OptionArea {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl OptionArea {
    pub fn unwrap(self) -> Area {
        Area {
            width: self.width.unwrap(),
            height: self.height.unwrap(),
        }
    }

    pub fn none() -> OptionArea {
        OptionArea {
            width: None,
            height: None,
        }
    }

    pub fn get_axis(self, axis: Axis) -> OptionSingleAxisArea {
        match axis {
            Axis::Horizontal => OptionSingleAxisArea {
                main_axis: self.width,
                axis,
            },
            Axis::Vertical => OptionSingleAxisArea {
                main_axis: self.height,
                axis,
            },
        }
    }

    pub fn axis_mut(&mut self, axis: Axis) -> Option<&mut u32> {
        match axis {
            Axis::Horizontal => self.width.as_mut(),
            Axis::Vertical => self.height.as_mut(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    pub fn cross(self) -> Axis {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SingleAxisArea {
    pub main_axis: u32,
    pub axis: Axis,
}

impl SingleAxisArea {
    pub fn dummy(&self) -> Area {
        match self.axis {
            Axis::Horizontal => area!(self.main_axis, u32::MAX),
            Axis::Vertical => area!(u32::MAX, self.main_axis),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OptionSingleAxisArea {
    pub main_axis: Option<u32>,
    pub axis: Axis,
}

impl OptionSingleAxisArea {
    pub fn unwrap(self) -> SingleAxisArea {
        SingleAxisArea {
            main_axis: self.main_axis.unwrap(),
            axis: self.axis,
        }
    }

    pub fn none(direction: Axis) -> OptionSingleAxisArea {
        OptionSingleAxisArea {
            main_axis: None,
            axis: direction,
        }
    }

    pub fn dummy(&self) -> OptionArea {
        match self.axis {
            Axis::Horizontal => OptionArea {
                width: self.main_axis,
                height: Some(u32::MAX),
            },
            Axis::Vertical => OptionArea {
                width: Some(u32::MAX),
                height: self.main_axis,
            },
        }
    }
}