use super::View;
use crate::activity::Activity;
use crate::layouts::Parent;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde_json::json;

#[derive(Copy, Clone)]
pub struct WebView<'a> {
    id: i32,
    activity: Activity<'a>,
}

impl<'a> WebView<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent) -> Self {
        let mut args = json!({});

        if let Some(id) = parent.id() {
            args["parent"] = json!(id);
        }

        let id = activity.send_recv_msg("createWebView", args);

        let web = WebView { id, activity };
        web.send_msg("setWidth", json!({"width": "MATCH_PARENT"}));
        web.send_msg("setHeight", json!({"height": "MATCH_PARENT"}));
        web
    }
}

impl<'a> WebView<'a> {
    pub fn set_data(&self, text: &str, mime: &str) {
        self.send_msg(
            "setData",
            json!({
                "base64": true,
                "mime": mime,
                "doc": BASE64_STANDARD.encode(text.as_bytes()),
            }),
        )
    }
}

impl<'a> View for WebView<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> Activity<'a> {
        self.activity
    }
}
