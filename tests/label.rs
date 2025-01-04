use tgui::TGui;

#[test]
fn label() {
    let tgui = TGui::new();
    let ui = tgui.new_activity(Default::default());
    ui.label(ui, "Hello", false, false);
    std::thread::sleep(std::time::Duration::from_secs(5));
}
