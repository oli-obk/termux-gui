use super::{Color, View};
use crate::layouts::Parent;
use crate::widgets::Widget;
use serde::Serialize;
use serde_json::json;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct Label<'a>(Widget<'a>);

impl<'a> Label<'a> {
    pub fn new(
        parent: impl Parent<'a>,
        text: &str,
        selectable_text: bool,
        clickable_links: bool,
    ) -> Self {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args<'a> {
            text: &'a str,
            selectable_text: bool,
            clickable_links: bool,
        }
        Label(Widget::new(
            "TextView",
            parent,
            Args {
                text,
                selectable_text,
                clickable_links,
            },
        ))
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
