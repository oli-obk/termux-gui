use tgui::event::Handler;
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

    let mut ehs: Handler = Handler::new(&tgui);

    ehs.new_activity(flags, |ui, ehs| {
        let layout = ui.linear_layout(true);

        let title = layout.label("Download Video", false, false);
        title.set_text_size(30);

        title.set_margin(5, None);

        layout.label("Video Link", false, false);
        layout.edit_text("", false, false, false, "text");

        layout.label("File Name", false, false);
        layout.edit_text("", false, false, false, "text");

        let buttons = layout.linear_layout(false);
        buttons.button("Download");
        let cancel = buttons.button("Cancel");

        ehs.on_click(cancel, move |_ehs| Ok(ui.finish()));
        Ok(())
    });

    ehs.handle_all_events().unwrap();
}
