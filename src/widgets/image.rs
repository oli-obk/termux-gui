use super::View;
use crate::activity::Activity;
use crate::layouts::Parent;
use base64::prelude::*;
use serde_json::json;
use std::io::Cursor;

pub struct ImageView<'a> {
    id: i32,
    activity: Activity<'a>,
}

impl<'a> ImageView<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent) -> Self {
        let mut args = json!({});

        if let Some(id) = parent.id() {
            args["parent"] = json!(id);
        }

        let id = activity.send_recv_msg("createImageView", args);

        ImageView { id, activity }
    }

    pub fn set_image(&self, img: &str) {
        let base_img = image::open(img).unwrap();
        let mut buff: Vec<u8> = Vec::new();
        base_img
            .write_to(&mut Cursor::new(&mut buff), image::ImageFormat::Png)
            .unwrap();
        let res_base64 = BASE64_STANDARD.encode(&buff);
        let args = json!({
            "img": res_base64
        });
        self.send_msg("setImage", args);
    }
}

impl<'a> View for ImageView<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> &Activity<'a> {
        &self.activity
    }
}
