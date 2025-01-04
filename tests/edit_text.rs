use tgui::TGui;

#[test]
fn edit_text() {
    let tgui = TGui::new();
    let ui = tgui.new_activity(Default::default());
    ui.default_edit_text(ui, "");
    std::thread::sleep(std::time::Duration::from_secs(5));
}
