use super::{Parent, ViewGroup};
use crate::widgets::Widget;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct RadioGroup<'a>(Widget<'a>);

impl<'a> RadioGroup<'a> {
    pub fn new(parent: impl Parent<'a>) -> Self {
        RadioGroup(Widget::new("RadioGroup", parent, ()))
    }
}

impl<'a> Deref for RadioGroup<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Widget<'a> {
        &self.0
    }
}

impl<'a> ViewGroup<'a> for RadioGroup<'a> {}
