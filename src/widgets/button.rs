use super::label::TextView;
use super::Widget;
use crate::layouts::Parent;
use serde::Serialize;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct Button<'a>(Widget<'a>);

impl<'a> Button<'a> {
    pub fn new(parent: impl Parent<'a>, text: &str) -> Self {
        #[derive(Serialize)]
        struct Args<'a> {
            text: &'a str,
        }

        Button(Widget::new("Button", parent, Args { text }))
    }
}

impl<'a> TextView<'a> for Button<'a> {}

impl<'a> Deref for Button<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
