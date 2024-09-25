use image::Pixel;

use crate::core::area::OptionSingleAxisArea;
use crate::core::component::Component;
use crate::core::constraint::Constraint;

pub fn allocate_area<P: Pixel>(mut area: OptionSingleAxisArea, components: &Vec<Box<dyn Component<P>>>) -> Vec<u32> {
    let mut allocated: Vec<u32> = Vec::with_capacity(components.len());

    for child in components.iter().filter(|c| c.constraint().width != Constraint::Maximized){
        let child_size = child.resolve_collision_size(area.clone());
        area.size.as_mut().map(|a| *a -= child_size);
        allocated.push(child_size);
    }

    for (i, child) in components.iter().enumerate().filter(|(_, c)| c.constraint().width == Constraint::Maximized) {
        let child_size = child.resolve_collision_size(area.clone());
        area.size.as_mut().map(|a| *a -= child_size);
        allocated.insert(i, child_size);
    }

    // println!("{:?}", allocated);

    allocated
}