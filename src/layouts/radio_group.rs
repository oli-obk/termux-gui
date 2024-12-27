use super::{send_recv_msg, RawFd, View, ViewGroup};
use serde_json::json;

pub struct RadioGroup<'a> {
    aid: i32,
    id: i32,
    sock: &'a RawFd,
}

impl<'a> RadioGroup<'a> {
    pub fn new(fd: &'a RawFd, aid: i32, parent: Option<i32>) -> Self {
        let mut args = json!({ "aid": aid });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }
        let id = send_recv_msg(fd, "createRadioGroup", args);

        RadioGroup { id, aid, sock: fd }
    }
}

impl<'a> View for RadioGroup<'a> {
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

impl<'a> ViewGroup for RadioGroup<'a> {}
