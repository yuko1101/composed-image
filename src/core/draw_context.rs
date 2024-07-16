use image::{ColorType, ImageBuffer, Pixel};
use crate::core::component::Component;
use crate::core::size::Size;

pub struct DrawContext<P: Pixel> {
    pub color_type: ColorType,
    pub absolute_position: (u32, u32),
    pub original_size: (u32, u32),
    pub height: u32,
    pub width: u32,

    pub new_buffer: ImageBuffer<P, Vec<P::Subpixel>>,
}

impl <P: Pixel> DrawContext<P> {
    pub(crate) fn child(&self, component: &Box<dyn Component<P>>) -> DrawContext<P> {
        if let Size::Constant(width, height) = component.size() {
            DrawContext {
                color_type: self.color_type,
                absolute_position: self.absolute_position,
                original_size: self.original_size,
                height,
                width,
                new_buffer: ImageBuffer::new(height, width)
            }
        } else {
            panic!("Child component must have a constant size")
        }
    }
}