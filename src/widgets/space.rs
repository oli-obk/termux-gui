use crate::layouts::Parent;
use crate::widgets::Widget;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct Space<'a>(Widget<'a>);

impl<'a> Space<'a> {
    pub fn new(parent: impl Parent<'a>) -> Self {
        Space(Widget::new("Space", parent, ()))
    }
}

impl<'a> Deref for Space<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
