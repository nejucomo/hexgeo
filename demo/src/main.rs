use eframe::egui::{
    CentralPanel, Context, MenuBar, Response, TopBottomPanel, Ui, ViewportBuilder, ViewportCommand,
    Widget,
};
use eframe::{Frame, NativeOptions, run_native};
use hexgeo::AxialBounds;
use hexgeo_egui::{HexOrientation, Wireframe};

mod select_menu;

fn main() -> eframe::Result<()> {
    run_native(
        env!("CARGO_PKG_NAME"),
        NativeOptions {
            viewport: ViewportBuilder::default().with_maximized(true),
            persist_window: false,
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}

struct App {
    bounds: AxialBounds,
    hexor: HexOrientation,
    radius: usize,
}

impl Default for App {
    fn default() -> Self {
        let radius = 3;
        Self {
            bounds: AxialBounds::new(radius),
            hexor: HexOrientation::default(),
            radius,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if self.radius != self.bounds.radius() {
            // The user changed the radius, reset the board:
            self.bounds = AxialBounds::new(self.radius);
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
        ui.add(Wireframe::new(&self.bounds, self.hexor))
    }
}
