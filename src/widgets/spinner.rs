use super::View;
use crate::activity::Activity;
use crate::layouts::Parent;
use crate::widgets::Widget;
use serde_json::json;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct Spinner<'a>(Widget<'a>);

impl<'a> Spinner<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent<'a>) -> Self {
        Spinner(Widget::new(activity, "Spinner", parent, ()))
    }

    pub fn set_list(&self, list: &[&str]) {
        let args = json!({
            "list": list
        });

        self.send_msg("setList", args);
    }
}

impl<'a> Deref for Spinner<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
