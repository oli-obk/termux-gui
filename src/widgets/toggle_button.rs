use crate::widgets::Serialize;
use super::compound_button::CompoundButton;
use super::label::TextView;
use crate::activity::Activity;
use crate::layouts::Parent;
use crate::widgets::Widget;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct ToggleButton<'a>(Widget<'a>);

impl<'a> ToggleButton<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent<'a>, text: &str, checked: bool) -> Self {
        #[derive(Serialize)]
        struct Args<'a> {
            text: &'a str,
            checked: bool,
        }

        ToggleButton(Widget::new(
            activity,
            "ToggleButton",
            parent,
            Args { text, checked },
        ))
    }
}

impl<'a> TextView<'a> for ToggleButton<'a> {}

impl<'a> CompoundButton<'a> for ToggleButton<'a> {}

impl<'a> Deref for ToggleButton<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
