use egui::{Color32, Frame, InnerResponse, Response, Sense, Stroke, Style, Ui, Widget};
use hexohexes::Board;

use crate::ext::AxialBoundsExt as _;
use crate::{AxialsExt as _, HexOrientation, Projector};

pub struct BoardFrame<'a, T> {
    board: &'a Board<T>,
    frame: Frame,
    hexor: HexOrientation,
}

pub type BoardResponse = Response;

impl<'a, T> BoardFrame<'a, T> {
    pub fn new(board: &'a Board<T>) -> Self {
        Self::new_style(board, &Style::default())
    }

    pub fn new_style(board: &'a Board<T>, style: &Style) -> Self {
        BoardFrame {
            board,
            frame: Frame::group(style),
            hexor: HexOrientation::default(),
        }
    }

    pub fn show(self, ui: &mut Ui) -> InnerResponse<BoardResponse> {
        self.frame.show(ui, |ui| self.show_inner(ui))
    }

    fn show_inner(self, ui: &mut Ui) -> BoardResponse {
        let resp = ui.allocate_rect(ui.max_rect(), Sense::hover());

        let projector = Projector::new(self.board.bounding_rect(self.hexor), ui.max_rect());
        let painter = ui.painter();

        for ax in self.board.iter_axials() {
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

impl<'a, T> Widget for BoardFrame<'a, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}
