use tgui::TGui;

#[test]
fn label() {
    let tgui = TGui::new();
    let ui = tgui.ui(Default::default());
    ui.label(ui.activity(), "Hello", false, false);
    std::thread::sleep(std::time::Duration::from_secs(5));
}
