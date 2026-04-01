use eframe::egui::{
    CentralPanel, Context, MenuBar, Response, TopBottomPanel, Ui, ViewportBuilder, ViewportCommand,
    Widget,
};
use eframe::{Frame, NativeOptions, run_native};
use hexohexes::Board;
use hexohexes_egui::{BoardWidget, HexOrientation};

mod select_menu;

fn main() -> eframe::Result<()> {
    let radius = 3;
    let app = App {
        board: Board::new_defaults(radius),
        hexor: HexOrientation::default(),
        radius,
    };

    run_native(
        env!("CARGO_PKG_NAME"),
        NativeOptions {
            viewport: ViewportBuilder::default().with_maximized(true),
            persist_window: false,
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(app))),
    )
}

struct App {
    board: Board<()>,
    hexor: HexOrientation,
    radius: usize,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if self.radius != self.board.radius() {
            // The user changed the radius, reset the board:
            self.board = Board::new_defaults(self.radius);
        }

        TopBottomPanel::top("my_panel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                    }
                });

                select_menu::add_with_type_name(ui, &mut self.hexor, {
                    use HexOrientation::*;

                    [FlatTop, PointyTop]
                });

                select_menu::add(ui, "radius", &mut self.radius, 0..=7);
            });
        });
        CentralPanel::default().show(ctx, |ui| ui.add(self));
    }
}

impl Widget for &mut App {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(BoardWidget::new(&self.board, self.hexor))
    }
}
