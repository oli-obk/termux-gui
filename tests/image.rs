#![cfg(feature = "image")]

use tgui::layouts::Parent as _;
use tgui::TGui;

#[test]
fn image() {
    let tgui = TGui::new();
    let ui = tgui.new_activity(Default::default());
    let img = ui.image_view();
    img.set_image("res/rust.png");
    std::thread::sleep(std::time::Duration::from_secs(5));
}
