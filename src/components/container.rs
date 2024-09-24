use image::Pixel;

use crate::core::area::OptionSingleAxisArea;
use crate::core::component::Component;
use crate::core::draw_context::DrawContext;
use crate::core::edge_insets::EdgeInsets;
use crate::core::constraint::AreaConstraint;
use crate::core::image_source::ImageSource;

pub struct Container<P: Pixel> {
    pub padding: EdgeInsets,
    pub margin: EdgeInsets,
    pub constraint: AreaConstraint,
    pub child: Option<Box<dyn Component<P>>>,
    pub background: Option<Box<dyn ImageSource<P>>>,
}

impl<P: Pixel> Component<P> for Container<P> {
    fn padding(&self) -> EdgeInsets {
        self.padding
    }

    fn margin(&self) -> EdgeInsets {
        self.margin
    }

    fn constraint(&self) -> AreaConstraint {
        self.constraint
    }

    fn draw_content(&self, context: &mut DrawContext<P>) {
        if let Some(child) = &self.child {
            context.draw_child(child);
        }
    }

    fn draw_background(&self, context: &mut DrawContext<P>) {
        if let Some(background) = &self.background {
            background.draw(context);
        }
    }

    fn children(&self) -> Vec<&Box<dyn Component<P>>> {
        return if let Some(child) = &self.child {
            vec![child]
        } else {
            vec![]
        }
    }

    fn resolve_children_size(&self, area: OptionSingleAxisArea) -> u32 {
        if let Some(child) = &self.child {
            child.resolve_collision_size(area)
        } else {
            0
        }
    }
}
