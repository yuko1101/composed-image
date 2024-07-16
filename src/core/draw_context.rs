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
        let padding = component.padding();
        let new_abs_pos = (self.absolute_position.0 + padding.left, self.absolute_position.1 + padding.top);

        let (width, height) = match component.size() {
            Size::Constant(width, height) => (width, height),
            Size::Maximized => (self.width - padding.left - padding.right, self.height - padding.top - padding.bottom),
            Size::Minimized => panic!("Size::Minimized is not implemented")
        };

        DrawContext {
                color_type: self.color_type,
                absolute_position: new_abs_pos,
                original_size: self.original_size,
                width,
                height,
                new_buffer: ImageBuffer::new(width, height)
        }
    }
}