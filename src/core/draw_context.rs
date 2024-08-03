use image::{ColorType, ImageBuffer, Pixel};
use crate::core::area::{Area, area};
use crate::core::component::Component;
use crate::core::edge_insets::EdgeInsets;
use crate::core::pos::{Pos, pos};

pub struct DrawContext<P: Pixel> {
    pub color_type: ColorType,
    pub abs_pos: Pos,
    pub original_size: Area,
    pub area: Area,

    pub image_buffer: ImageBuffer<P, Vec<P::Subpixel>>,
}

impl<P: Pixel> DrawContext<P> {
    // `this` should be a content context of the current component
    pub fn child(&self, child: &Box<dyn Component<P>>) -> DrawContext<P> {
        self.custom_child(child, self.area)
    }

    pub fn custom_child(&self, child: &Box<dyn Component<P>>, area: Area) -> DrawContext<P> {
        let child_size = child.resolve_collision_size(area.into_option());

        DrawContext {
            color_type: self.color_type,
            abs_pos: self.abs_pos,
            original_size: self.original_size,
            area: child_size,
            image_buffer: ImageBuffer::new(child_size.width, child_size.height)
        }
    }

    pub fn with_size(&self, area: Area) -> DrawContext<P> {
        DrawContext {
            color_type: self.color_type,
            abs_pos: self.abs_pos,
            original_size: self.original_size,
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
            original_size: self.original_size,
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