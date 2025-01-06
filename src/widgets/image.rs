use super::View;
use crate::activity::Activity;
use crate::layouts::Parent;
use crate::widgets::Widget;
use base64::prelude::*;
use serde_json::json;
use std::io::Cursor;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct ImageView<'a>(Widget<'a>);

impl<'a> ImageView<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent<'a>) -> Self {
        ImageView(Widget::new(activity, "ImageView", parent, ()))
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

impl<'a> Deref for ImageView<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
