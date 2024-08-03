use image::Pixel;
use crate::core::area::{Area, area, Axis, OptionArea};
use crate::core::component::Component;
use crate::core::draw_context::DrawContext;
use crate::core::edge_insets::EdgeInsets;
use crate::core::pos::pos;
use crate::core::size::{Constraint, Size};
use crate::core::util;

pub struct Row<P: Pixel> {
    pub padding: EdgeInsets,
    pub margin: EdgeInsets,
    pub size: Size,
    pub children: Vec<Box<dyn Component<P>>>,
}

impl<P: Pixel> Component<P> for Row<P> {
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
        let allocated = util::allocate_area(context.area, Axis::Horizontal, &self.children);

        let mut offset = 0;
        for (i, child) in self.children.iter().enumerate() {
            let abs_pos = pos![offset, 0];
            let mut child_context = context.with_size(allocated[i]);
            child_context.move_offset(abs_pos);
            child.draw_component(&mut child_context);
            context.overlay(&child_context);
            offset += allocated[i].width as i32;
        }
    }

    fn draw_background(&self, _: &mut DrawContext<P>) {
    }

    fn children(&self) -> Vec<&Box<dyn Component<P>>> {
        self.children.iter().collect::<Vec<_>>()
    }

    fn resolve_children_size(&self, mut area: OptionArea) -> Area {
        let mut width = 0;
        let mut height = 0;
        for child in self.children.iter().filter(|c| c.content_size().width != Constraint::Maximized){
            let child_size = child.resolve_collision_size(area.clone());
            width += child_size.width;
            height = height.max(child_size.height);
            if let Some(w) = area.width.as_mut() {
                *w -= child_size.width;
            }
        }

        let maximized_list = self.children.iter().filter(|c| c.content_size().width == Constraint::Maximized).collect::<Vec<_>>();

        for child in maximized_list {
            let child_size = child.resolve_collision_size(area.clone());
            width += child_size.width;
            height = height.max(child_size.height);
            if let Some(w) = area.width.as_mut() {
                *w -= child_size.width;
            }
        }

        area!(width, height)
    }
}