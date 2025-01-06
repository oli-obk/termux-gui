use super::View;
use crate::layouts::Parent;
use crate::widgets::Widget;
use base64::prelude::*;
use serde::Serialize;
use std::io::Cursor;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct ImageView<'a>(Widget<'a>);

impl<'a> ImageView<'a> {
    pub fn new(parent: impl Parent<'a>) -> Self {
        ImageView(Widget::new("ImageView", parent, ()))
    }

    pub fn set_image(&self, img: &str) {
        let base_img = image::open(img).unwrap();
        let mut buff: Vec<u8> = Vec::new();
        base_img
            .write_to(&mut Cursor::new(&mut buff), image::ImageFormat::Png)
            .unwrap();
        let img = BASE64_STANDARD.encode(&buff);
        #[derive(Serialize)]
        struct Args {
            img: String,
        }
        self.send_msg("setImage", Args { img });
    }
}

impl<'a> Deref for ImageView<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
