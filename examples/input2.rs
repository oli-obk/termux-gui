use tgui::event::{self, Event, Widget::Click};
use tgui::layouts::Parent as _;
use tgui::widgets::{label::TextView, View};
use tgui::TGui;

fn main() {
    let tgui = TGui::new();
    let flags = tgui::activity::Flags {
        dialog: true,
        cancel_outside: true,
        ..Default::default()
    };

    let ui = tgui.new_activity(flags);
    let layout = ui.linear_layout(true);

    let title = layout.default_label("Input Demo");
    title.set_text_size(30);

    title.set_margin(5, None);

    let switch = layout.default_switch("Switch");

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
            } if id == switch.get_id() => ui.finish(),
            _ => {}
        }
    }
}
