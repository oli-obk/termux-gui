use tgui::utils::Color;
use tgui::widgets::label::TextView;
use tgui::TGui;

fn main() {
    let tgui = TGui::new();
    let ui = tgui.new_activity(Default::default());
    let label = ui.label(ui, "Hello", false, false);
    std::thread::sleep(std::time::Duration::from_secs(5));
    label.set_text("Bye World");
    label.set_text_color(Color::from_rgb(160, 200, 240));
    std::thread::sleep(std::time::Duration::from_secs(5));
}
