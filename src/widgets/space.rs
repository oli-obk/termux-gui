use crate::activity::Activity;
use crate::layouts::Parent;
use crate::widgets::Widget;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct Space<'a>(Widget<'a>);

impl<'a> Space<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent<'a>) -> Self {
        Space(Widget::new(activity, "Space", parent, ()))
    }
}

impl<'a> Deref for Space<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
