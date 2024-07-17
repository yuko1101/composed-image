use image::Pixel;
use crate::core::component::Component;
use crate::core::draw_context::DrawContext;
use crate::core::edge_insets::EdgeInsets;

pub struct Column<P: Pixel> {
    pub padding: EdgeInsets,
    pub margin: EdgeInsets,
    pub children: Vec<Box<dyn Component<P>>>,
}

impl<P: Pixel> Component<P> for Column<P> {
    fn padding(&self) -> EdgeInsets {
        self.padding   
    }

    fn margin(&self) -> EdgeInsets {
        self.margin
    }
    
    fn draw_content(&self, context: &mut DrawContext<P>) {
        let mut y = 0;
        for child in &self.children {
            let mut child_context = context.custom_child(child, (context.width, context.height - y), (context.absolute_position.0, context.absolute_position.1 + y));
            child.draw_component(&mut child_context);
            context.overlay(&child_context);
            y += child_context.height;
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
            width = width.max(child_width);
            height += child_height;
            if let Some(area) = area.as_mut() {
                area.1 -= child_height;
            }
        }
        (width, height)
    }
}