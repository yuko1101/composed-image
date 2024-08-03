pub mod core;
pub mod components;

#[cfg(test)]
mod tests {
    use image::{ColorType, Rgba};

    use crate::components::container::{Container, ContainerBackground};
    use crate::components::row::Row;
    use crate::core::{component::Component, draw_context::DrawContext, edge_insets::EdgeInsets, size::Size};
    use crate::core::size::Constraint;

    #[test]
    fn it_works() {
        let child1 = Container {
            padding: EdgeInsets::zero(),
            margin: EdgeInsets::all(5),
            size: Size {
                width: Constraint::Maximized,
                height: Constraint::Constant(50),
            },
            child: None,
            background: ContainerBackground {
                color: Rgba([255, 0, 0, 255]),
            },
        };

        let child2 = Container {
            padding: EdgeInsets::zero(),
            margin: EdgeInsets::all(5),
            size: Size {
                width: Constraint::Constant(10),
                height: Constraint::Constant(20),
            },
            child: None,
            background: ContainerBackground {
                color: Rgba([0, 0, 255, 255]),
            },
        };

        let row = Row {
            padding: EdgeInsets::zero(),
            margin: EdgeInsets::zero(),
            size: Size {
                width: Constraint::Constant(100),
                height: Constraint::Minimized,
            },
            children: vec![Box::new(child1), Box::new(child2)],
        };

        let base_component = Box::new(Container {
            padding: EdgeInsets::zero(),
            margin: EdgeInsets::zero(),
            size: Size {
                width: Constraint::Constant(100),
                height: Constraint::Minimized,
            },
            child: Some(Box::new(row)),
            background: ContainerBackground {
                color: Rgba([0, 100, 0, 255]),
            },

        });

        let ctx: DrawContext<Rgba<u8>> = base_component.start_draw(ColorType::Rgba8);

        ctx.image_buffer.save("test.png").unwrap();
    }
}
