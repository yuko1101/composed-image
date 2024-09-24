use image::Pixel;
use crate::core::draw_context::DrawContext;

pub trait ImageSource<P: Pixel> {
    fn draw(&self, context: &mut DrawContext<P>);
}

pub struct SingleColorSource<P: Pixel> {
    pub color: P,
}
impl<P: Pixel> ImageSource<P> for SingleColorSource<P> {
    fn draw(&self, context: &mut DrawContext<P>) {
        for (_, _, pixel) in &mut context.image_buffer.enumerate_pixels_mut() {
            *pixel = self.color;
        }
    }
}
