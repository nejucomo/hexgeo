use derive_new::new;
use eframe::egui::{Rect, Response, Scene, Ui, Widget};

use crate::axext::AxialsExt;
use crate::gameboard::GameBoard;

#[derive(Debug, new)]
pub struct BoardWidget<'a> {
    board: &'a GameBoard,
    view: &'a mut Rect,
}

impl<'a> Widget for BoardWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        Scene::new()
            .show(ui, self.view, |ui| {
                let p = ui.painter();
                for ax in self.board.iter_axials() {
                    ax.paint_center(p);
                }
            })
            .response
    }
}
