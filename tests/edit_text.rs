use tgui::TGui;

#[test]
fn edit_text() {
    let tgui = TGui::new();
    let ui = tgui.ui(Default::default());
    ui.default_edit_text(ui.activity(), "");
    std::thread::sleep(std::time::Duration::from_secs(5));
}
