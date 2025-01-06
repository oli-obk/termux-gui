use std::ops::Deref;
use crate::widgets::Widget;
use super::{Parent, ViewGroup};
use crate::activity::Activity;
use serde_json::json;

#[derive(Copy, Clone)]
pub struct RadioGroup<'a>(Widget<'a>);

impl<'a> RadioGroup<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent<'a>) -> Self {
        let args = json!({});

        RadioGroup(Widget::new(activity, "RadioGroup", parent, args))
    }
}

impl<'a> Deref for RadioGroup<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Widget<'a> {
        &self.0
    }
}

impl<'a> ViewGroup<'a> for RadioGroup<'a> {}
