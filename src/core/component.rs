use image::{ColorType, Pixel};
use crate::core::draw_context::DrawContext;
use crate::core::edge_insets::EdgeInsets;
use crate::core::size::{Constraint, Size};

/*
content size (children size): (width, height)
background size (component visual size): (width + padding.left + padding.right, height + padding.top + padding.bottom)
collision size (component collision size): (width + padding.left + padding.right + margin.left + margin.right, height + padding.top + padding.bottom + margin.top + margin.bottom)
*/

pub trait Component<P: Pixel> {
    fn padding(&self) -> EdgeInsets {
        EdgeInsets::zero()
    }

    fn margin(&self) -> EdgeInsets {
        EdgeInsets::zero()
    }

    fn content_size(&self) -> Size {
        Size {
            width: Constraint::Maximized,
            height: Constraint::Maximized,
        }
    }

    fn draw_content(&self, context: &mut DrawContext<P>);
    fn draw_background(&self, context: &mut DrawContext<P>);

    fn draw_component(&self, collusion_context: &mut DrawContext<P>) {
        let mut background_context = collusion_context.narrow(&self.margin());
        let mut content_context = background_context.narrow(&self.padding());
        self.draw_background(&mut background_context);
        collusion_context.overlay(&background_context);
        self.draw_content(&mut content_context);
        collusion_context.overlay(&content_context);
    }

    fn resolve_collision_size(&self, area: Option<(u32, u32)>) -> (u32, u32) {
        // TODO: this can call self.padding() and self.margin() multiple times, maybe should be cached
        let visual_size = self.resolve_visual_size(area);
        let margin = self.margin();
        (visual_size.0 + margin.left + margin.right, visual_size.1 + margin.top + margin.bottom)
    }

    fn resolve_visual_size(&self, area: Option<(u32, u32)>) -> (u32, u32) {
        // TODO: this can call self.padding() multiple times, maybe should be cached
        let content_size = self.resolve_content_size(area);
        let padding = self.padding();
        (content_size.0 + padding.left + padding.right, content_size.1 + padding.top + padding.bottom)
    }

    fn resolve_content_size(&self, area: Option<(u32, u32)>) -> (u32, u32) {
        let size = self.content_size();
        (self.resolve_constraint(&size.width, area, true), self.resolve_constraint(&size.height, area, false))
    }

    fn resolve_constraint(&self, constraint: &Constraint, area: Option<(u32, u32)>, is_horizontal: bool) -> u32 {
        match constraint {
            Constraint::Maximized => {
                if area.is_none() {
                    panic!("Maximized component must have a parent size");
                }
                let area = area.unwrap();
                let margin = self.margin();
                let padding = self.padding();

                if is_horizontal {
                    area.0 - margin.left - margin.right - padding.left - padding.right
                } else {
                    area.1 - margin.top - margin.bottom - padding.top - padding.bottom
                }

            },
            Constraint::Minimized => {
                let children_size = self.resolve_children_size(area);
                if is_horizontal {
                    children_size.0
                } else {
                    children_size.1
                }
            },
            Constraint::Constant(value) => *value,
        }
    }

    fn children(&self) -> Vec<&Box<dyn Component<P>>>;
    fn resolve_children_size(&self, area: Option<(u32, u32)>) -> (u32, u32);

    fn start_draw(&self, color_type: ColorType) -> DrawContext<P> {
        let (width, height) = self.resolve_collision_size(None);
        let mut collision_context = DrawContext {
            color_type,
            absolute_position: (0, 0),
            original_size: (width, height),
            width,
            height,
            image_buffer: image::ImageBuffer::new(width, height),
        };

        self.draw_component(&mut collision_context);
        collision_context

    }
}
