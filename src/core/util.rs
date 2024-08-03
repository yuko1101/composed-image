use image::Pixel;
use crate::core::area::{Area, Axis};
use crate::core::component::Component;
use crate::core::size::Constraint;

pub fn allocate_area<P: Pixel>(mut area: Area, axis: Axis, components: &Vec<Box<dyn Component<P>>>) -> Vec<Area> {
    let mut allocated: Vec<Area> = Vec::with_capacity(components.len());

    for child in components.iter().filter(|c| c.content_size().width != Constraint::Maximized){
        let child_size = child.resolve_collision_size(area.into_option());
        *area.axis_mut(axis) -= child_size.axis(axis);
        allocated.push(child_size);
    }

    for (i, child) in components.iter().enumerate().filter(|(_, c)| c.content_size().width == Constraint::Maximized) {
        let child_size = child.resolve_collision_size(area.into_option());
        *area.axis_mut(axis) -= child_size.axis(axis);
        allocated.insert(i, child_size);
    }

    allocated
}