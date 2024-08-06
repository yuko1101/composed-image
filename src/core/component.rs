use image::{ColorType, Pixel};

use crate::core::area::{area, Axis, OptionSingleAxisArea};
use crate::core::draw_context::DrawContext;
use crate::core::edge_insets::EdgeInsets;
use crate::core::pos::pos;
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

    fn resolve_collision_size(&self, area: OptionSingleAxisArea) -> u32 {
        let visual_size = self.resolve_visual_size(area);
        let margin = self.margin();
        visual_size + margin.sum_axis(area.axis)
    }

    fn resolve_visual_size(&self, area: OptionSingleAxisArea) -> u32 {
        let content_size = self.resolve_content_size(area);
        let padding = self.padding();
        content_size + padding.sum_axis(area.axis)
    }

    fn resolve_content_size(&self, area: OptionSingleAxisArea) -> u32 {
        let size = self.content_size();
        self.resolve_constraint(size.get_axis(area.axis), area)
    }

    fn resolve_constraint(&self, constraint: Constraint, area: OptionSingleAxisArea) -> u32 {
        match constraint {
            Constraint::Maximized => {
                if area.main_axis.is_none() {
                    panic!("Maximized component must have a parent size");
                }
                let area = area.unwrap();
                let margin = self.margin();
                let padding = self.padding();

                area.main_axis - margin.sum_axis(area.axis) - padding.sum_axis(area.axis)
            },
            Constraint::Minimized => self.resolve_children_size(area),
            Constraint::Constant(value) => value,
        }
    }

    fn children(&self) -> Vec<&Box<dyn Component<P>>>;
    fn resolve_children_size(&self, area: OptionSingleAxisArea) -> u32;

    fn start_draw(&self, color_type: ColorType) -> DrawContext<P> {
        let width = self.resolve_collision_size(OptionSingleAxisArea::none(Axis::Horizontal));
        let height = self.resolve_collision_size(OptionSingleAxisArea::none(Axis::Vertical));
        let area = area![width, height];
        let mut collision_context = DrawContext {
            color_type,
            abs_pos: pos![0, 0],
            original_size: area,
            area,
            image_buffer: image::ImageBuffer::new(area.width, area.height),
        };

        self.draw_component(&mut collision_context);
        collision_context

    }
}
