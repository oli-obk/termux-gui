use super::View;
use crate::layouts::Parent;
use crate::widgets::Widget;
use serde::Serialize;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct ProgressBar<'a>(Widget<'a>);

impl<'a> ProgressBar<'a> {
    pub fn new(parent: impl Parent<'a>) -> Self {
        ProgressBar(Widget::new("ProgressBar", parent, ()))
    }

    pub fn set_progress(&self, progress: u8) {
        #[derive(Serialize)]
        struct Args {
            progress: u8,
        }

        self.send_msg("setProgress", Args { progress });
    }
}

impl<'a> Deref for ProgressBar<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
