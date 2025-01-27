use super::{Parent, ViewGroup};
use crate::widgets::Widget;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct FrameLayout<'a>(Widget<'a>);

impl<'a> FrameLayout<'a> {
    #[must_use]
    pub fn new(parent: impl Parent<'a>) -> Self {
        FrameLayout(Widget::new("FrameLayout", parent, ()))
    }
}

impl<'a> ViewGroup<'a> for FrameLayout<'a> {}

impl<'a> Deref for FrameLayout<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Widget<'a> {
        &self.0
    }
}
