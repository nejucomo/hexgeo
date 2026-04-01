use std::ops::RangeInclusive;

use egui::{Pos2, Rect, remap};

#[derive(Debug)]
pub struct Projector {
    x_from: RangeInclusive<f32>,
    y_from: RangeInclusive<f32>,
    x_to: RangeInclusive<f32>,
    y_to: RangeInclusive<f32>,
}

impl Projector {
    pub fn new(source: Rect, dest: Rect) -> Self {
        let Rect {
            min: source_min,
            max: source_max,
        } = source;

        let Rect {
            min: dest_min,
            max: dest_max,
        } = dest;

        Projector {
            x_from: source_min.x..=source_max.x,
            y_from: source_min.y..=source_max.y,
            x_to: dest_min.x..=dest_max.x,
            y_to: dest_min.y..=dest_max.y,
        }
    }

    /// Project a point in source coordinates to dest coordinates
    pub fn project(&self, src: Pos2) -> Pos2 {
        Pos2::new(
            remap(src.x, self.x_from.clone(), self.x_to.clone()),
            remap(src.y, self.y_from.clone(), self.y_to.clone()),
        )
    }
}
