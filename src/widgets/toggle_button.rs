use super::compound_button::CompoundButton;
use super::label::TextView;
use super::RawFd;
use super::{send_recv_msg, View};
use serde_json::json;

pub struct ToggleButton<'a> {
    id: i32,
    aid: i32,
    sock: &'a RawFd,
    check: bool,
}

impl<'a> ToggleButton<'a> {
    pub fn new(fd: &'a RawFd, aid: i32, parent: Option<i32>, text: &str, check: bool) -> Self {
        let mut args = json!({
            "aid": aid,
            "text": text,
            "checked": check
        });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }

        let id = send_recv_msg(fd, "createToggleButton", args);
        ToggleButton {
            id,
            aid,
            sock: fd,
            check,
        }
    }
}

impl<'a> TextView for ToggleButton<'a> {}

impl<'a> CompoundButton for ToggleButton<'a> {
    fn check(&mut self, set: bool) {
        self.check = set;
    }
}

impl<'a> View for ToggleButton<'a> {
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
