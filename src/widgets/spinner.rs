use super::View;
use crate::layouts::Parent;
use crate::widgets::Serialize;
use crate::widgets::Widget;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct Spinner<'a>(Widget<'a>);

impl<'a> Spinner<'a> {
    pub fn new(parent: impl Parent<'a>) -> Self {
        Spinner(Widget::new("Spinner", parent, ()))
    }

    pub fn set_list(&self, list: &[&str]) {
        #[derive(Serialize)]
        struct Args<'a> {
            list: &'a [&'a str],
        }

        self.send_msg("setList", Args { list });
    }
}

impl<'a> Deref for Spinner<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
