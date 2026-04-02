use derive_new::new;
use egui::{Color32, Response, Sense, Stroke, Ui, Widget};
use hexgeo::geom::{DHO, HexOrientation};
use hexgeo::radial::RadialIndexMap;

use crate::projector::Projector;

/// A [Widget] for [RadialIndexMap] for displaying a wireframe of hexes with hover highlighting support
#[derive(Debug, new)]
pub struct Wireframe<'a> {
    bounds: &'a RadialIndexMap,
    hexor: DHO,
}

impl<'a> Widget for Wireframe<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        // TODO: Configurable sizing behavior
        let resp = ui.allocate_rect(ui.max_rect(), Sense::hover());

        // TODO: Configurable aspect ratio
        let projector = Projector::new(
            self.hexor.disc_bounding_rect(self.bounds.radius()),
            ui.max_rect(),
        );
        let painter = ui.painter();

        for ax in self.bounds.iter_axials() {
            painter.circle_stroke(
                projector.project(ax.center_pos(self.hexor)),
                10.0,
                Stroke {
                    width: 1.0,
                    color: Color32::WHITE,
                },
            );
        }

        resp
    }
}
