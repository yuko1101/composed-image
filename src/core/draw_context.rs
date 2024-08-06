use image::{ColorType, ImageBuffer, Pixel};

use crate::core::area::{Area, area, Axis, OptionArea};
use crate::core::component::Component;
use crate::core::edge_insets::EdgeInsets;
use crate::core::pos::{Pos, pos};

pub struct DrawContext<P: Pixel> {
    pub color_type: ColorType,
    pub abs_pos: Pos,
    pub original_area: Area,
    pub area: Area,

    pub image_buffer: ImageBuffer<P, Vec<P::Subpixel>>,
}

impl<P: Pixel> DrawContext<P> {
    // `this` should be a content context of the current component
    pub fn child(&self, child: &Box<dyn Component<P>>) -> DrawContext<P> {
        self.custom_child(child, self.area.into_option())
    }

    pub fn custom_child(&self, child: &Box<dyn Component<P>>, area: OptionArea) -> DrawContext<P> {
        let child_width = child.resolve_collision_size(area.get_axis(Axis::Horizontal));
        let child_height = child.resolve_collision_size(area.get_axis(Axis::Vertical));

        DrawContext {
            color_type: self.color_type,
            abs_pos: self.abs_pos,
            original_area: self.original_area,
            area: area![child_width, child_height],
            image_buffer: ImageBuffer::new(child_width, child_height)
        }
    }

    pub fn with_area(&self, area: Area) -> DrawContext<P> {
        DrawContext {
            color_type: self.color_type,
            abs_pos: self.abs_pos,
            original_area: self.original_area,
            area,
            image_buffer: ImageBuffer::new(area.width, area.height)
        }
    }

    pub fn narrow(&self, edge_insets: &EdgeInsets) -> DrawContext<P> {
        let mut new_abs_pos = self.abs_pos.clone();
        new_abs_pos.offset(pos![edge_insets.left as i32, edge_insets.top as i32]);

        let width = self.area.width - edge_insets.left - edge_insets.right;
        let height = self.area.height - edge_insets.top - edge_insets.bottom;

        DrawContext {
            color_type: self.color_type,
            abs_pos: new_abs_pos,
            original_area: self.original_area,
            area: area![width, height],
            image_buffer: ImageBuffer::new(width, height)
        }
    }

    pub fn overlay(&mut self, child_context: &DrawContext<P>) {
        let relative_pos = child_context.abs_pos - self.abs_pos;
        image::imageops::overlay(&mut self.image_buffer, &child_context.image_buffer, relative_pos.x as i64, relative_pos.y as i64);
    }

    pub fn draw_child(&mut self, child: &Box<dyn Component<P>>) {
        let mut child_context = self.child(&child);
        child.draw_component(&mut child_context);
        image::imageops::overlay(&mut self.image_buffer, &child_context.image_buffer, child_context.abs_pos.x as i64, child_context.abs_pos.y as i64);
    }

    pub fn move_offset(&mut self, offset: Pos) {
        self.abs_pos.offset(offset);
    }
}