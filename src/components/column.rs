use image::Pixel;

use crate::core::area::{area, Axis, OptionSingleAxisArea};
use crate::core::component::Component;
use crate::core::draw_context::DrawContext;
use crate::core::edge_insets::EdgeInsets;
use crate::core::pos::pos;
use crate::core::size::{Constraint, Size};
use crate::core::util;

pub struct Column<P: Pixel> {
    pub padding: EdgeInsets,
    pub margin: EdgeInsets,
    pub size: Size,
    pub children: Vec<Box<dyn Component<P>>>,
}

impl<P: Pixel> Component<P> for Column<P> {
    fn padding(&self) -> EdgeInsets {
        self.padding
    }

    fn margin(&self) -> EdgeInsets {
        self.margin
    }

    fn content_size(&self) -> Size {
        self.size
    }

    fn draw_content(&self, context: &mut DrawContext<P>) {
        let allocated = util::allocate_area(context.area.into_option().get_axis(Axis::Vertical), &self.children);

        let mut offset = 0;
        for (i, child) in self.children.iter().enumerate() {
            let abs_pos = pos![0, offset];
            let child_cross_axis = child.resolve_collision_size(context.area.into_option().get_axis(Axis::Horizontal));
            let mut child_context = context.with_size(area![child_cross_axis, allocated[i]]);
            child_context.move_offset(abs_pos);
            child.draw_component(&mut child_context);
            context.overlay(&child_context);
            offset += allocated[i] as i32;
        }
    }

    fn draw_background(&self, _: &mut DrawContext<P>) {
    }

    fn children(&self) -> Vec<&Box<dyn Component<P>>> {
        self.children.iter().collect::<Vec<_>>()
    }

    fn resolve_children_size(&self, mut area: OptionSingleAxisArea) -> u32 {
        let mut size = 0;
        for child in self.children.iter().filter(|c| c.content_size().width != Constraint::Maximized){
            let child_size = child.resolve_collision_size(area.clone());
            if area.axis == Axis::Horizontal {
                size = size.max(child_size);
            } else {
                size += child_size;
                area.main_axis.as_mut().map(|a| *a -= child_size);
            }
        }

        let maximized_list = self.children.iter().filter(|c| c.content_size().width == Constraint::Maximized).collect::<Vec<_>>();

        for child in maximized_list {
            let child_size = child.resolve_collision_size(area.clone());
            if area.axis == Axis::Horizontal {
                size = size.max(child_size);
            } else {
                size += child_size;
                area.main_axis.as_mut().map(|a| *a -= child_size);
            }
        }

        size
    }
}