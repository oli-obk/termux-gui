use super::RawFd;
use super::{send_recv_msg, View};
use serde_json::json;

pub struct Space<'a> {
    id: i32,
    aid: i32,
    sock: &'a RawFd,
}

impl<'a> Space<'a> {
    pub fn new(fd: &'a RawFd, aid: i32, parent: Option<i32>) -> Self {
        let mut args = json!({
            "aid": aid,
        });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }

        let id = send_recv_msg(fd, "createSpace", args);

        Space { id, aid, sock: fd }
    }
}

impl<'a> View for Space<'a> {
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
