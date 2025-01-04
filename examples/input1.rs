use tgui::event::Handler;
use tgui::widgets::{label::TextView, View};
use tgui::TGui;

fn main() {
    let tgui = TGui::new();
    let flags = tgui::activity::Flags {
        dialog: true,
        cancel_outside: true,
        ..Default::default()
    };

    let mut ehs: Handler = Handler::new(&tgui);

    let ui = ehs.new_activity(flags);

    let layout = ui.linear_layout(ui, true);

    let title = ui.label(layout, "Download Video", false, false);
    title.set_text_size(30);

    title.set_margin(5, None);

    ui.label(layout, "Video Link", false, false);
    ui.edit_text(layout, "", false, false, false, "text");

    ui.label(layout, "File Name", false, false);
    ui.edit_text(layout, "", false, false, false, "text");

    let buttons = ui.linear_layout(layout, false);
    ui.button(buttons, "Download");
    let cancel = ui.button(buttons, "Cancel");

    ehs.on_click(cancel, |_ehs| Ok(ui.finish()));

    ehs.handle_all_events().unwrap();
}
