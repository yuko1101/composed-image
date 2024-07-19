pub mod core;
pub mod components;

#[cfg(test)]
mod tests {
    use image::{ColorType, Rgba};

    use crate::components::column::Column;
    use crate::components::container::{Container, ContainerBackground};
    use crate::core::{component::Component, draw_context::DrawContext, edge_insets::EdgeInsets, size::Size};
    use crate::core::size::Constraint;

    #[test]
    fn it_works() {
        let child1 = Container {
            padding: EdgeInsets::zero(),
            margin: EdgeInsets::all(5),
            size: Size {
                width: Constraint::Maximized,
                height: Constraint::Constant(20),
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
                width: Constraint::Constant(40),
                height: Constraint::Maximized,
            },
            child: None,
            background: ContainerBackground {
                color: Rgba([0, 0, 255, 255]),
            },
        };

        let col = Column {
            padding: EdgeInsets::zero(),
            margin: EdgeInsets::zero(),
            size: Size {
                width: Constraint::Maximized,
                height: Constraint::Maximized,
            },
            children: vec![Box::new(child1), Box::new(child2)],
        };

        let base_component = Box::new(Container {
            padding: EdgeInsets::zero(),
            margin: EdgeInsets::zero(),
            size: Size {
                width: Constraint::Constant(100),
                height: Constraint::Constant(100),
            },
            child: Some(Box::new(col)),
            background: ContainerBackground {
                color: Rgba([0, 255, 0, 255]),
            },

        });

        let ctx: DrawContext<Rgba<u8>> = base_component.start_draw(ColorType::Rgba8);

        ctx.image_buffer.save("test.png").unwrap();
    }
}
