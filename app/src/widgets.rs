use derive_new::new;
use eframe::egui::{Color32, Rect, Response, Scene, Ui, Widget};

use crate::axext::AxialsExt;
use crate::gameboard::GameBoard;

#[derive(Debug, new)]
pub struct BoardWidget<'a> {
    board: &'a GameBoard,
    view: &'a mut Rect,
}

impl<'a> Widget for BoardWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let pixscale = {
            let r = ui.max_rect();

            // TODO: Balance aspect ratio of view with that of hex board
            r.width().min(r.height()) / (4 * self.board.radius()) as f32
        };

        Scene::new()
            .show(ui, self.view, |ui| {
                let p = ui.painter();
                for ax in self.board.iter_axials() {
                    p.circle_filled(ax.center_pos() * pixscale, pixscale / 5.0, Color32::WHITE);
                }
            })
            .response
    }
}
