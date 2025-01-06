use std::ops::Deref;
use crate::widgets::Widget;
use super::{Color, View};
use crate::activity::Activity;
use crate::layouts::Parent;
use serde_json::json;

#[derive(Copy, Clone)]
pub struct Label<'a>(Widget<'a>);

impl<'a> Label<'a> {
    pub fn new(
        activity: Activity<'a>,
        parent: impl Parent<'a>,
        text: &str,
        selectable_text: bool,
        clickable_links: bool,
    ) -> Self {
        let args = json!({
            "text": text,
            "selectableText": selectable_text,
            "clickableLinks": clickable_links
        });
        Label(Widget::new(activity, "TextView", parent, args))
    }
}

pub trait TextView<'a>: View<'a> + Sized {
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

impl<'a> TextView<'a> for Label<'a> {}

impl<'a> Deref for Label<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
