pub mod core;
pub mod components;

#[cfg(test)]
mod tests {
    use image::{ColorType, Rgba};
    use crate::core::{component::Component, edge_insets::EdgeInsets, size::Size, draw_context::DrawContext};
    use crate::components::container::{Container, ContainerBackground};
    use crate::core::size::Size::Constant;

    #[test]
    fn it_works() {
        let child = Container {
            padding: EdgeInsets::zero(),
            margin: EdgeInsets::all(5),
            size: Size::Maximized,
            child: None,
            background: ContainerBackground {
                color: Rgba([255, 0, 0, 255]),
            },
        };

        let base_component = Box::new(Container {
            padding: EdgeInsets::zero(),
            margin: EdgeInsets::zero(),
            size: Constant(40, 60),
            child: Some(Box::new(child)),
            background: ContainerBackground {
                color: Rgba([0, 255, 0, 255]),
            },

        });

        let ctx: DrawContext<Rgba<u8>> = base_component.start_draw(ColorType::Rgba8);

        ctx.image_buffer.save("test.png").unwrap();
    }
}
