use image::{ColorType, Pixel};
use crate::core::area::{area, Area, Axis, OptionArea, OptionSingleAxisArea};
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

    fn resolve_collision_size(&self, area: OptionArea) -> Area {
        let visual_size = self.resolve_visual_size(area);
        let margin = self.margin();
        area!(visual_size.width + margin.left + margin.right, visual_size.height + margin.top + margin.bottom)
    }

    fn resolve_visual_size(&self, area: OptionArea) -> Area {
        let content_size = self.resolve_content_size(area);
        let padding = self.padding();
        area!(content_size.width + padding.left + padding.right, content_size.height + padding.top + padding.bottom)
    }

    fn resolve_content_size(&self, area: OptionArea) -> Area {
        let size = self.content_size();
        area!(self.resolve_constraint(&size.width, area.get_axis(Axis::Horizontal)), self.resolve_constraint(&size.height, area.get_axis(Axis::Vertical)))
    }

    fn resolve_constraint(&self, constraint: &Constraint, area: OptionSingleAxisArea) -> u32 {
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
            Constraint::Minimized => {
                let children_size = self.resolve_children_size(area.dummy());
                children_size.single_axis(area.axis).main_axis
            },
            Constraint::Constant(value) => *value,
        }
    }

    fn children(&self) -> Vec<&Box<dyn Component<P>>>;
    fn resolve_children_size(&self, area: OptionArea) -> Area;

    fn start_draw(&self, color_type: ColorType) -> DrawContext<P> {
        let area = self.resolve_collision_size(OptionArea::none());
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
