use image::Pixel;
use crate::core::component::Component;
use crate::core::draw_context::DrawContext;
use crate::core::edge_insets::EdgeInsets;
use crate::core::size::Size;

pub struct Row<P: Pixel> {
    pub padding: EdgeInsets,
    pub margin: EdgeInsets,
    pub size: Size,
    pub children: Vec<Box<dyn Component<P>>>,
}

impl<P: Pixel> Component<P> for Row<P> {
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
        let mut x = 0;
        for child in &self.children {
            let mut child_context = context.custom_child(child, (context.width - x, context.height), (context.absolute_position.0 + x, context.absolute_position.1));
            child.draw_component(&mut child_context);
            context.overlay(&child_context);
            x += child_context.width;
        }
    }

    fn draw_background(&self, _: &mut DrawContext<P>) {
    }

    fn children(&self) -> Vec<&Box<dyn Component<P>>> {
        self.children.iter().collect::<Vec<_>>()
    }

    fn resolve_children_size(&self, mut area: Option<(u32, u32)>) -> (u32, u32) {
        let mut width = 0;
        let mut height = 0;
        for child in &self.children {
            let (child_width, child_height) = child.resolve_collision_size(area.clone());
            width += child_width;
            height = height.max(child_height);
            if let Some(area) = area.as_mut() {
                area.0 -= child_width;
            }
        }
        (width, height)
    }
}