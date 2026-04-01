use eframe::egui::layers::ShapeIdx;
use eframe::egui::{Color32, Painter, Pos2};
use extension_traits::extension;
use hexohexes::Axials;

#[extension(pub trait AxialsExt)]
impl Axials {
    fn center_pos(self) -> Pos2 {
        let qf = self.q as f32;
        let rf = self.r as f32;

        Pos2::new(
            3f32.sqrt().mul_add(qf, 3f32.sqrt() / 2.0 * rf),
            3.0 / 2.0 * rf,
        )
    }

    fn paint_center(self, painter: &Painter) -> ShapeIdx {
        painter.circle_filled(self.center_pos(), 0.2, Color32::WHITE)
    }
}
