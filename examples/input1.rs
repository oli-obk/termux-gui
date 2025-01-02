use tgui::event::{Handler, Widget::Click};
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

    let layout = ui.linear_layout(None, true);

    let title = ui.label(Some(&layout), "Download Video", false, false);
    title.set_text_size(30);

    title.set_margin(5, None);

    ui.label(Some(&layout), "Video Link", false, false);
    ui.edit_text(Some(&layout), "", false, false, false, "text");

    ui.label(Some(&layout), "File Name", false, false);
    ui.edit_text(Some(&layout), "", false, false, false, "text");

    let buttons = ui.linear_layout(Some(&layout), false);
    ui.button(Some(&buttons), "Download");
    let cancel = ui.button(Some(&buttons), "Cancel");

    ehs.add_widget(&cancel, |kind, _ehs| {
        Ok(if let Click { .. } = kind {
            ui.finish()
        })
    });

    ehs.handle_all_events().unwrap();
}
