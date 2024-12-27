use super::Vec2;
use super::{send_recv_msg, RawFd, View, ViewGroup};
use serde_json::json;

pub struct TabLayout<'a> {
    aid: i32,
    id: i32,
    sock: &'a RawFd,
}

impl<'a> TabLayout<'a> {
    pub fn new(fd: &'a RawFd, aid: i32, parent: Option<i32>) -> Self {
        let mut args = json!({ "aid": aid });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }
        let id = send_recv_msg(fd, "createTabLayout", args);

        TabLayout { id, aid, sock: fd }
    }

    pub fn set_scroll_position(&self, pos: Vec2<u16>, smooth: bool) {
        let args = json!({
           "x": pos.x,
           "y": pos.y,
           "soft": smooth
        });
        self.send_msg("setScrollPosition", args);
    }

    pub fn get_scroll_position(&self) -> Vec2<u16> {
        let ret = self.send_recv_msg("getScrollPosition", ());
        let x: u16 = ret["x"].to_string().parse().unwrap();
        let y: u16 = ret["y"].to_string().parse().unwrap();
        Vec2 { x, y }
    }

    pub fn set_list(&self, list: &[&str]) {
        let args = json!({
            "list": list
        });

        self.send_msg("setList", args);
    }
}

impl<'a> View for TabLayout<'a> {
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

impl<'a> ViewGroup for TabLayout<'a> {}
