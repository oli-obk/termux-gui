use serde::Serialize;
use super::View;
use crate::layouts::Parent;
use crate::widgets::Widget;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde_json::json;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct WebView<'a>(Widget<'a>);

impl<'a> WebView<'a> {
    pub fn new(parent: impl Parent<'a>) -> Self {
        let web = WebView(Widget::new("WebView", parent, ()));
        web.send_msg("setWidth", json!({"width": "MATCH_PARENT"}));
        web.send_msg("setHeight", json!({"height": "MATCH_PARENT"}));
        web
    }
}

impl<'a> WebView<'a> {
    pub fn set_data(&self, text: &str, mime: &str) {
        #[derive(Serialize)]
        struct Args<'a> {
            base64: bool,
            mime: &'a str,
            doc: String,
        }

        self.send_msg(
            "setData",
            Args {
                base64: true,
                mime,
                doc: BASE64_STANDARD.encode(text.as_bytes()),
            },
        )
    }
}

impl<'a> Deref for WebView<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
