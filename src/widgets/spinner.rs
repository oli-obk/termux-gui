use super::{construct_message, send_recv_msg, RawFd, View};
use serde_json::json;

pub struct Spinner<'a> {
    aid: i32,
    id: i32,
    sock: &'a RawFd,
}

impl<'a> Spinner<'a> {
    pub fn new(fd: &'a RawFd, aid: i32, parent: Option<i32>) -> Self {
        let mut args = json!({ "aid": aid });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }
        let id = send_recv_msg(fd, construct_message("createSpinner", &args));

        Spinner { id, aid, sock: fd }
    }

    pub fn set_list(&self, list: &[&str]) {
        let args = json!({
            "aid": &self.aid,
            "id": &self.id,
            "list": list
        });

        self.send_msg(construct_message("setList", &args));
    }
}

impl<'a> View for Spinner<'a> {
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
