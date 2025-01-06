use serde::Serialize;
use super::label::TextView;
use super::Widget;
use crate::activity::Activity;
use crate::layouts::Parent;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct Button<'a>(Widget<'a>);

impl<'a> Button<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent<'a>, text: &str) -> Self {
        #[derive(Serialize)]
        struct Args<'a> {
            text: &'a str,
        }

        Button(Widget::new(activity, "Button", parent, Args { text }))
    }
}

impl<'a> TextView<'a> for Button<'a> {}

impl<'a> Deref for Button<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
