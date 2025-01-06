use super::View;
use crate::activity::Activity;
use crate::layouts::Parent;
use crate::widgets::Widget;
use serde_json::json;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct ProgressBar<'a>(Widget<'a>);

impl<'a> ProgressBar<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent<'a>) -> Self {
        ProgressBar(Widget::new(activity, "ProgressBar", parent, ()))
    }

    pub fn set_progress(&self, progress: u8) {
        let args = json!({"progress": progress});
        self.send_msg("createProgressBar", args);
    }
}

impl<'a> Deref for ProgressBar<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
