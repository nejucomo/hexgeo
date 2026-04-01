use eframe::egui::Ui;

pub fn add<T, I>(ui: &mut Ui, label: &str, state: &mut T, options: I)
where
    T: std::fmt::Debug + PartialEq,
    I: IntoIterator<Item = T>,
{
    ui.menu_button(label, |ui| {
        for option in options {
            let text = format!("{:?}", &option);
            if ui.selectable_value(state, option, text).clicked() {
                ui.close();
                return;
            }
        }
    });
}

pub fn add_with_type_name<T, I>(ui: &mut Ui, state: &mut T, options: I)
where
    T: std::fmt::Debug + PartialEq,
    I: IntoIterator<Item = T>,
{
    add(
        ui,
        std::any::type_name::<T>().rsplit_once("::").unwrap().1,
        state,
        options,
    )
}
