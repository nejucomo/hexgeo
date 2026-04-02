mod ext;

use eframe::egui::{
    CentralPanel, Context, MenuBar, Response, TopBottomPanel, Ui, ViewportBuilder, ViewportCommand,
    Widget,
};
use eframe::{Frame, NativeOptions, run_native};
use hexgeo::geom::DHO;
use hexgeo::radial::RadialIndexMap;
use hexgeo_egui::Wireframe;

use crate::ext::UiExt as _;

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
    bounds: RadialIndexMap,
    dho: DHO,
    radius: usize,
}

impl Default for App {
    fn default() -> Self {
        let radius = 3;
        Self {
            bounds: RadialIndexMap::new(radius),
            dho: DHO::default(),
            radius,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if self.radius != self.bounds.radius() {
            // The user changed the radius, reset the board:
            self.bounds = RadialIndexMap::new(self.radius);
        }

        TopBottomPanel::top("my_panel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                    }
                });

                ui.menu_choice("Orientation", &mut self.dho, [DHO::FlatTop, DHO::PointyTop]);
                ui.menu_choice("Radius", &mut self.radius, 0..=7);
            });
        });
        CentralPanel::default().show(ctx, |ui| ui.add(self));
    }
}

impl Widget for &mut App {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(Wireframe::new(&self.bounds, self.dho))
    }
}
