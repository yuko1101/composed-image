use image::{ColorType, ImageBuffer, Pixel};
use crate::core::component::Component;
use crate::core::edge_insets::EdgeInsets;

pub struct DrawContext<P: Pixel> {
    pub color_type: ColorType,
    pub absolute_position: (u32, u32),
    pub original_size: (u32, u32),
    pub height: u32,
    pub width: u32,

    pub buffer_layer: ImageBuffer<P, Vec<P::Subpixel>>,
}

impl <P: Pixel> DrawContext<P> {
    // `this` should be a content context of the current component
    pub fn child(&self, child: &Box<dyn Component<P>>) -> DrawContext<P> {
        let (width, height) = child.resolve_collision_size((self.width, self.height));

        DrawContext {
            color_type: self.color_type,
            absolute_position: self.absolute_position,
            original_size: self.original_size,
            width,
            height,
            buffer_layer: ImageBuffer::new(width, height)
        }
    }

    pub fn narrow(&self, edge_insets: &EdgeInsets) -> DrawContext<P> {
        let new_abs_pos = (self.absolute_position.0 + edge_insets.left, self.absolute_position.1 + edge_insets.top);

        let width = self.width - edge_insets.left - edge_insets.right;
        let height = self.height - edge_insets.top - edge_insets.bottom;

        DrawContext {
            color_type: self.color_type,
            absolute_position: new_abs_pos,
            original_size: self.original_size,
            width,
            height,
            buffer_layer: ImageBuffer::new(width, height)
        }
    }

    pub fn overlay(&mut self, child_context: &DrawContext<P>) {
        let relative_pos = (child_context.absolute_position.0 - self.absolute_position.0, child_context.absolute_position.1 - self.absolute_position.1);
        image::imageops::overlay(&mut self.buffer_layer, &child_context.buffer_layer, relative_pos.0 as i64, relative_pos.1 as i64);
    }
}