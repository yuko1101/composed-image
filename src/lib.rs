pub mod core;
pub mod components;

#[cfg(test)]
mod tests {
    use image::{ColorType, Rgba};
    use crate::core::{component::Component, edge_insets::EdgeInsets, size::Size, draw_context::DrawContext};
    use crate::components::container::{Container, ContainerBackground};
    use crate::components::row::Row;

    #[test]
    fn it_works() {
        let child1 = Container {
            padding: EdgeInsets::zero(),
            margin: EdgeInsets::all(5),
            size: Size::Constant(10, 20),
            child: None,
            background: ContainerBackground {
                color: Rgba([255, 0, 0, 255]),
            },
        };

        let child2 = Container {
            padding: EdgeInsets::zero(),
            margin: EdgeInsets::all(5),
            size: Size::Maximized,
            child: None,
            background: ContainerBackground {
                color: Rgba([0, 0, 255, 255]),
            },
        };

        let row = Row {
            padding: EdgeInsets::zero(),
            margin: EdgeInsets::zero(),
            children: vec![Box::new(child1), Box::new(child2)],
        };

        let base_component = Box::new(Container {
            padding: EdgeInsets::zero(),
            margin: EdgeInsets::zero(),
            size: Size::Constant(40, 60),
            child: Some(Box::new(row)),
            background: ContainerBackground {
                color: Rgba([0, 255, 0, 255]),
            },

        });

        let ctx: DrawContext<Rgba<u8>> = base_component.start_draw(ColorType::Rgba8);

        ctx.image_buffer.save("test.png").unwrap();
    }
}
