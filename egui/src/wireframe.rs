use derive_new::new;
use egui::{Response, Sense, Ui, Widget};
use hexgeo::geom::{DHO, HexOrientation};
use hexgeo::radial::RadialIndexMap;

use crate::projector::Projector;
use crate::{HexUi, HexWidget as _};

/// A [Widget] for [RadialIndexMap] for displaying a wireframe of hexes with hover highlighting support
#[derive(Debug, new)]
pub struct Wireframe<'a> {
    bounds: &'a RadialIndexMap,
    dho: DHO,
}

impl<'a> Widget for Wireframe<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        // TODO: Configurable sizing behavior
        let resp = ui.allocate_rect(ui.max_rect(), Sense::hover());

        // TODO: Configurable aspect ratio
        let projector = Projector::new(
            self.dho.disc_bounding_rect(self.bounds.radius()),
            ui.max_rect(),
        );

        for ax in self.bounds.iter_axials() {
            ().hex_ui(HexUi::new(ui, &projector, self.dho, ax));
        }

        resp
    }
}
