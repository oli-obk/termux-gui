use tgui::event::{self, Event, Widget::Click};
use tgui::widgets::{label::TextView, View};
use tgui::TGui;

fn main() {
    let tgui = TGui::new();
    let flags = tgui::activity::Flags {
        dialog: true,
        cancel_outside: true,
        ..Default::default()
    };

    let ui = tgui.ui(flags);
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

    loop {
        match tgui.event().unwrap() {
            Event::Activity {
                kind: event::Activity::Destroy,
                finishing: true,
                ..
            } => break,
            Event::Widget {
                id,
                kind: Click { .. },
                ..
            } if id == cancel.get_id() => ui.finish(),
            other => {
                eprintln!("{other:#?}")
            }
        }
    }
}
