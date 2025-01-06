use super::compound_button::CompoundButton;
use super::label::TextView;
use crate::layouts::Parent;
use crate::widgets::Serialize;
use crate::widgets::Widget;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct Switch<'a>(Widget<'a>);

impl<'a> Switch<'a> {
    pub fn new(parent: impl Parent<'a>, text: &str, checked: bool) -> Self {
        #[derive(Serialize)]
        struct Args<'a> {
            text: &'a str,
            checked: bool,
        }

        Switch(Widget::new("Switch", parent, Args { checked, text }))
    }
}

impl<'a> TextView<'a> for Switch<'a> {}

impl<'a> CompoundButton<'a> for Switch<'a> {}

impl<'a> Deref for Switch<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
