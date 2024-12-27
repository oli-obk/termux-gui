use super::RawFd;
use super::{construct_message, send_recv_msg, View};
use base64::prelude::*;
use serde_json::json;
use std::io::Cursor;

pub struct ImageView<'a> {
    id: i32,
    aid: i32,
    sock: &'a RawFd,
}

impl<'a> ImageView<'a> {
    pub fn new(fd: &'a RawFd, aid: i32, parent: Option<i32>) -> Self {
        let mut args = json!({ "aid": aid });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }

        let id = send_recv_msg(fd, construct_message("createImageView", &args));

        ImageView { id, aid, sock: fd }
    }

    pub fn set_image(&self, img: &str) {
        let base_img = image::open(img).unwrap();
        let mut buff: Vec<u8> = Vec::new();
        base_img
            .write_to(&mut Cursor::new(&mut buff), image::ImageFormat::Png)
            .unwrap();
        let res_base64 = BASE64_STANDARD.encode(&buff);
        let args = json!({
            "aid": &self.aid,
            "id": self.id,
            "img": res_base64
        });
        self.send_msg(construct_message("setImage", &args));
    }
}

impl<'a> View for ImageView<'a> {
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
