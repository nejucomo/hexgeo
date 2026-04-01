use derive_new::new;
use eframe::egui::{CentralPanel, Context, Rect, Response, Ui, ViewportBuilder, Widget};
use eframe::{Frame, NativeOptions, run_native};

use crate::gameboard::GameBoard;
use crate::widgets::BoardWidget;

const HEX_RADIUS: usize = 3;

#[derive(new)]
pub(crate) struct App {
    board: GameBoard,
    boardview: Rect,
}

impl App {
    pub(crate) fn run() -> eframe::Result<()> {
        run_native(
            env!("CARGO_PKG_NAME"),
            NativeOptions {
                viewport: ViewportBuilder::default().with_maximized(true),
                persist_window: false,
                ..Default::default()
            },
            Box::new(|cc| Ok(Box::new(Self::init(cc)))),
        )
    }

    fn init(cc: &eframe::CreationContext<'_>) -> Self {
        log::trace!("{:#?}", cc.egui_ctx.style());

        let viewbound = (1 + HEX_RADIUS) as f32;
        Self::new(
            GameBoard::new_defaults(HEX_RADIUS),
            dbg!(Rect::from_x_y_ranges(
                -viewbound..=viewbound,
                -viewbound..=viewbound
            )),
        )
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| ui.add(self));
    }
}

impl Widget for &mut App {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(BoardWidget::new(&self.board, &mut self.boardview))
    }
}
