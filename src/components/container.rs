use image::Pixel;
use crate::core::component::Component;
use crate::core::draw_context::DrawContext;
use crate::core::edge_insets::EdgeInsets;
use crate::core::size::Size;

pub struct Container<P: Pixel> {
    pub padding: EdgeInsets,
    pub margin: EdgeInsets,
    pub size: Size,
    pub child: Option<Box<dyn Component<P>>>,
    pub background: ContainerBackground<P>,
}

impl<P: Pixel> Component<P> for Container<P> {
    fn padding(&self) -> EdgeInsets {
        self.padding
    }

    fn margin(&self) -> EdgeInsets {
        self.margin
    }

    fn content_size(&self) -> Size {
        self.size
    }

    fn draw_content(&self, context: &mut DrawContext<P>) {
        if let Some(child) = &self.child {
            context.draw_child(child);
        }
    }

    fn draw_background(&self, context: &mut DrawContext<P>) {
        for (_, _, pixel) in &mut context.image_buffer.enumerate_pixels_mut() {
            *pixel = self.background.color;
        }
    }

    fn children(&self) -> Vec<&Box<dyn Component<P>>> {
        return if let Some(child) = &self.child {
            vec![child]
        } else {
            vec![]
        }
    }

    fn resolve_children_size(&self, area: Option<(u32, u32)>) -> (u32, u32) {
        if let Some(child) = &self.child {
            child.resolve_collision_size(area)
        } else {
            (0, 0)
        }
    }
}

pub struct ContainerBackground<P: Pixel> {
    pub color: P,
}