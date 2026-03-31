use derive_new::new;
use eframe::egui::{CentralPanel, Context, Response, Ui, ViewportBuilder, Widget};
use eframe::{Frame, NativeOptions, run_native};

#[derive(new)]
pub(crate) struct App {}

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
        Self::new()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| ui.add(self));
    }
}

impl Widget for &mut App {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label("hi")
    }
}
