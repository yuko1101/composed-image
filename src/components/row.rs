use image::Pixel;
use crate::core::component::Component;
use crate::core::draw_context::DrawContext;
use crate::core::edge_insets::EdgeInsets;
use crate::core::size::{Constraint, Size};

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
        let mut drawed: Vec<DrawContext<P>> = vec![];

        // draw children
        let mut remaining_width = context.width;
        for child in self.children.iter().filter(|c| c.content_size().width != Constraint::Maximized){
            // TODO: fix absolute position if possible, this causes passing wrong absolute position to children
            let mut child_context = context.custom_child(child, (remaining_width, context.height), context.absolute_position);
            child.draw_component(&mut child_context);
            remaining_width -= child_context.width;
            drawed.push(child_context);
        }

        for (i, child) in self.children.iter().enumerate().filter(|(_, c)| c.content_size().width == Constraint::Maximized) {
            // TODO: fix absolute position if possible, this causes passing wrong absolute position to children
            let mut child_context = context.custom_child(child, (remaining_width, context.height), context.absolute_position);
            child.draw_component(&mut child_context);
            remaining_width -= child_context.width;
            drawed.insert(i, child_context);
        }

        // overlay children
        let mut x = 0;
        for mut child_context in drawed {
            child_context.move_offset((x, 0));
            x += child_context.width;
            context.overlay(&child_context);
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
        for child in self.children.iter().filter(|c| c.content_size().width != Constraint::Maximized){
            let (child_width, child_height) = child.resolve_collision_size(area.clone());
            width += child_width;
            height = height.max(child_height);
            if let Some(area) = area.as_mut() {
                area.0 -= child_width;
            }
        }

        for child in self.children.iter().filter(|c| c.content_size().width == Constraint::Maximized){
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