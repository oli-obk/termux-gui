use super::label::TextView;
use super::RawFd;
use super::{send_recv_msg, View};
use serde_json::json;

pub struct EditText<'a> {
    id: i32,
    aid: i32,
    sock: &'a RawFd,
}

impl<'a> EditText<'a> {
    pub fn new(
        fd: &'a RawFd,
        aid: i32,
        parent: Option<i32>,
        text: &str,
        single_line: bool,
        line: bool,
        block_input: bool,
        ty: &str,
    ) -> Self {
        let mut args = json!({
            "aid": aid,
            "text": text,
            "singleline": single_line,
            "line": line,
            "blockinput": block_input,
            "type": ty
        });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }

        let id = send_recv_msg(fd, "createEditText", args);

        EditText { id, aid, sock: fd }
    }

    pub fn show_cursor(&self, show: bool) {
        let args = json!({
            "show": show
        });
        self.send_msg("showCursor", args);
    }
}

impl<'a> TextView for EditText<'a> {}

impl<'a> View for EditText<'a> {
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
