use super::compound_button::CompoundButton;
use super::label::TextView;
use super::Widget;
use crate::activity::Activity;
use crate::layouts::Parent;
use serde_json::json;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct CheckBox<'a>(Widget<'a>);

impl<'a> CheckBox<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent<'a>, text: &str, check: bool) -> Self {
        let args = json!({

            "text": text,
            "checked": check
        });

        CheckBox(Widget::new(activity, "CheckBox", parent, args))
    }
}

impl<'a> TextView<'a> for CheckBox<'a> {}

impl<'a> CompoundButton<'a> for CheckBox<'a> {}

impl<'a> Deref for CheckBox<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
