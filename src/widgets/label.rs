use super::RawFd;
use super::{send_recv_msg, Color, View};
use serde_json::json;

pub struct Label<'a> {
    id: i32,
    aid: i32,
    sock: &'a RawFd,
}

impl<'a> Label<'a> {
    pub fn new(
        fd: &'a RawFd,
        aid: i32,
        parent: Option<i32>,
        text: &str,
        selectable_text: bool,
        clickable_links: bool,
    ) -> Self {
        let mut args = json!({
            "aid": aid,
            "text": text,
            "selectableText": selectable_text,
            "clickableLinks": clickable_links
        });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }

        let id = send_recv_msg(fd, "createTextView", args);

        Label { id, aid, sock: fd }
    }
}

pub trait TextView: View + Sized {
    fn set_text_size(&self, size: u8) {
        let args = json!({
            "size": size
        });

        self.send_msg("setTextSize", args);
    }

    fn set_text(&self, text: &str) {
        let args = json!({
            "text": text
        });

        self.send_msg("setText", args);
    }

    fn get_text(&self) -> String {
        self.send_recv_msg("getText", ()).to_string()
    }

    fn set_text_color(&self, color: Color) {
        let args = json!({
            "color": color.to_u32()
        });

        self.send_msg("setTextColor", args);
    }

    fn set_text_event(&self, send: bool) {
        let args = json!({
            "send": send
        });

        self.send_msg("sendTextEvent", args);
    }
}

impl TextView for Label<'_> {}

impl View for Label<'_> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_aid(&self) -> i32 {
        self.aid
    }

    fn get_sock(&self) -> &RawFd {
        self.sock
    }
}
