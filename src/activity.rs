use super::connection::{construct_message, send_msg, send_recv_msg};
use super::{RawFd, AF};
use serde_json::json;

pub struct Activity {
    pub tid: i32,
    pub aid: i32,
}

/// Behavior when the soft keyboard shows up.
#[derive(Copy, Clone, serde::Serialize)]
#[serde(rename = "lowercase")]
pub enum InputMode {
    Resize,
    Pan,
}

impl Activity {
    pub fn new(main: &RawFd, tid: Option<i32>, flags: AF) -> Self {
        let mut args = json!({
            "dialog": flags.contains(AF::DIALOG),
            "pip": flags.contains(AF::PIP),
            "lockscreen": flags.contains(AF::LOCK_SCREEN),
            "canceloutside": flags.contains(AF::CANCEL_OUTSIDE),
            "overlay": flags.contains(AF::OVERLAY)
        });

        if let Some(val) = tid {
            args["tid"] = json!(val);
        }

        let [aid, tid] = send_recv_msg(main, construct_message("newActivity", &args));

        Activity { tid, aid }
    }

    pub fn finish(&self, main: &RawFd) {
        let args = json!({
            "aid": &self.aid
        });
        send_msg(main, construct_message("finishActivity", &args));
    }

    pub fn set_input_mode(&self, main: &RawFd, mode: InputMode) {
        let args = json!({
            "aid": &self.aid,
            "mode": mode
        });

        send_msg(main, construct_message("setInputMode", &args));
    }
}
