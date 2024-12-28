use super::TGui;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct Activity<'a> {
    #[expect(dead_code)]
    tid: i32,
    aid: i32,
    gui: &'a TGui,
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
#[serde(rename_all = "lowercase")]
pub enum InputMode {
    Resize,
    Pan,
}

#[derive(Serialize)]
struct WithId<T: Serialize> {
    aid: i32,
    #[serde(flatten)]
    params: T,
}

impl<'a> Activity<'a> {
    pub fn new(gui: &'a TGui, flags: Flags) -> Self {
        let [aid, tid] = gui.send_recv_msg("newActivity", flags);

        Activity { tid, aid, gui }
    }

    pub fn send_recv_msg<T: for<'d> Deserialize<'d>>(
        &self,
        method: &str,
        params: impl Serialize,
    ) -> T {
        self.gui.send_recv_msg(
            method,
            WithId {
                params,
                aid: self.aid,
            },
        )
    }

    pub fn send_msg(&self, method: &str, params: impl Serialize) {
        self.gui.send_msg(
            method,
            WithId {
                params,
                aid: self.aid,
            },
        );
    }

    pub fn finish(&self) {
        self.send_msg("finishActivity", ());
    }

    pub fn set_input_mode(&self, mode: InputMode) {
        let args = json!({
            "mode": mode
        });

        self.send_msg("setInputMode", args);
    }

    pub fn aid(&self) -> i32 {
        self.aid
    }
}
