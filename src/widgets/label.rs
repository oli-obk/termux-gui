use super::{Color, View};
use crate::activity::Activity;
use crate::layouts::Parent;
use serde_json::json;

pub struct Label<'a> {
    id: i32,
    activity: Activity<'a>,
}

impl<'a> Label<'a> {
    pub fn new(
        activity: Activity<'a>,
        parent: impl Parent,
        text: &str,
        selectable_text: bool,
        clickable_links: bool,
    ) -> Self {
        let mut args = json!({
            "text": text,
            "selectableText": selectable_text,
            "clickableLinks": clickable_links
        });

        if let Some(id) = parent.id() {
            args["parent"] = json!(id);
        }

        let id = activity.send_recv_msg("createTextView", args);

        Label { id, activity }
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
        self.send_recv_msg("getText", ())
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

impl<'a> View for Label<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> &Activity<'a> {
        &self.activity
    }
}
