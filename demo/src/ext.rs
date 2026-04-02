use eframe::egui::Ui;
use extension_traits::extension;

#[extension(pub trait UiExt)]
impl Ui {
    fn menu_choice<T, I>(&mut self, label: &str, state: &mut T, options: I)
    where
        T: std::fmt::Debug + PartialEq,
        I: IntoIterator<Item = T>,
    {
        self.menu_button(label, |ui| {
            for option in options {
                let text = format!("{:?}", &option);
                if ui.selectable_value(state, option, text).clicked() {
                    ui.close();
                    return;
                }
            }
        });
    }
}
