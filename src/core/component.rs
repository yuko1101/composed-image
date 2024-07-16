use image::Pixel;
use crate::core::draw_context::DrawContext;
use crate::core::padding::Padding;
use crate::core::size::Size;

pub trait Component<P: Pixel> {
    fn padding(&self) -> Padding;

    fn size(&self) -> Size;

    fn draw(&self, context: &mut DrawContext<P>);
}
