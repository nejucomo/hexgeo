use egui::{Color32, Stroke};
use hexgeo::Axials;
use hexgeo::geom::HexOrientation as _;

use crate::HexUi;

/// A [HexWidget] is responsible for rendering and interaction for a specific hex
pub trait HexWidget {
    fn hex_ui(self, hui: HexUi<'_>);
}

impl HexWidget for () {
    fn hex_ui(self, hui: HexUi<'_>) {
        let stroke = hui.style().visuals.widgets.inactive.fg_stroke;
        hui.path(hui.orientation().vertices(), stroke, true);

        if hui.axials() == Axials::ORIGIN {
            let stroke = Stroke {
                width: 1.0,
                color: Color32::GREEN,
            };

            hui.line([(-1.0, 0.0), (1.0, 0.0)], stroke);
            hui.line([(0.0, -1.0), (0.0, 1.0)], stroke);

            let r = hui.orientation().bounding_rect();
            hui.path(
                [
                    r.left_top(),
                    r.right_top(),
                    r.right_bottom(),
                    r.left_bottom(),
                ],
                Stroke {
                    width: 1.0,
                    color: Color32::BLUE,
                },
                true,
            );
        }
    }
}
