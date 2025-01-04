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

    let ui = tgui.new_activity(flags);
    let layout = ui.linear_layout(ui, true);

    let title = ui.default_label(layout, "Input Demo");
    title.set_text_size(30);

    title.set_margin(5, None);

    let switch = ui.default_switch(layout, "Switch");

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
