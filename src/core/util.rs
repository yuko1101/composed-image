use image::imageops;
use image::Pixel;
use crate::core::component::Component;
use crate::core::draw_context::DrawContext;

pub fn overlay<P: Pixel>(child: &Box<dyn Component<P>>, context: &mut DrawContext<P>) {
    let mut child_context = context.child(&child);
    child.draw_component(&mut child_context);
    imageops::overlay(&mut context.buffer_layer, &child_context.buffer_layer, child_context.absolute_position.0 as i64, child_context.absolute_position.1 as i64);
}