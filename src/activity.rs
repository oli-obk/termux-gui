use super::connection::{construct_message, send_msg, send_recv_msg};
use super::RawFd;
use serde::Serialize;
use serde_json::json;

pub struct Activity {
    pub tid: i32,
    pub aid: i32,
}

#[derive(Serialize, Default)]
pub struct Flags {
    pub dialog: bool,
    pub pip: bool,
    pub cancel_outside: bool,
    pub lock_screen: bool,
    pub overlay: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tid: Option<i32>,
}

/// Behavior when the soft keyboard shows up.
#[derive(Copy, Clone, serde::Serialize)]
#[serde(rename = "lowercase")]
pub enum InputMode {
    Resize,
    Pan,
}

impl Activity {
    pub fn new(main: &RawFd, flags: Flags) -> Self {
        let [aid, tid] = send_recv_msg(main, construct_message("newActivity", &flags));

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
