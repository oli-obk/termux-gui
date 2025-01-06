use super::compound_button::CompoundButton;
use super::label::TextView;
use super::Widget;
use crate::activity::Activity;
use crate::layouts::Parent;
use serde::Serialize;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct CheckBox<'a>(Widget<'a>);

impl<'a> CheckBox<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent<'a>, text: &str, checked: bool) -> Self {
        #[derive(Serialize)]
        struct Args<'a> {
            text: &'a str,
            checked: bool,
        }

        CheckBox(Widget::new(
            activity,
            "CheckBox",
            parent,
            Args { text, checked },
        ))
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
