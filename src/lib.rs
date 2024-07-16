pub mod core;
pub mod components;

#[cfg(test)]
mod tests {
    use image::{ColorType, Rgba};
    use image::imageops::overlay;
    use crate::core::{component::Component, padding::Padding, size::Size, draw_context::DrawContext};

    #[test]
    fn it_works() {
        struct TestComponent2;
        impl Component<Rgba<u8>> for TestComponent2 {
            fn padding(&self) -> Padding {
                Padding::zero()
            }

            fn size(&self) -> Size {
                Size::Constant(10, 30)
            }

            fn draw(&self, context: &mut DrawContext<Rgba<u8>>) {
                let mut b = &mut context.new_buffer;
                for (x, y, pixel) in b.enumerate_pixels_mut() {
                    *pixel = Rgba([255, 0, 0, 255]);
                }
            }
        }

        struct TestComponent;
        impl Component<Rgba<u8>> for TestComponent {
            fn padding(&self) -> Padding {
                Padding::zero()
            }

            fn size(&self) -> Size {
                Size::Constant(20, 40)
            }

            fn draw(&self, context: &mut DrawContext<Rgba<u8>>) {
                let mut b = &mut context.new_buffer;
                for (x, y, pixel) in b.enumerate_pixels_mut() {
                    *pixel = Rgba([0, 0, 255, 255]);
                }

                let child: Box<dyn Component<Rgba<u8>>> = Box::new(TestComponent2);
                let mut child_context = context.child(&child);
                child.draw(&mut child_context);
                overlay(&mut context.new_buffer, &child_context.new_buffer, child_context.absolute_position.0 as i64, child_context.absolute_position.1 as i64);
            }
        }

        let base_component = Box::new(TestComponent);
        let mut draw_context: DrawContext<Rgba<u8>> = DrawContext {
            color_type: ColorType::Rgb8,
            absolute_position: (0, 0),
            original_size: (20, 40),
            height: 40,
            width: 20,
            new_buffer: image::ImageBuffer::new(40, 20),
        };

        base_component.draw(&mut draw_context);

        draw_context.new_buffer.save("test.png").unwrap();
    }
}
