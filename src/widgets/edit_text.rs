use super::label::TextView;
use super::View;
use super::Widget;
use crate::layouts::Parent;
use serde::Serialize;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct EditText<'a>(Widget<'a>);

impl<'a> EditText<'a> {
    pub fn new(
        parent: impl Parent<'a>,
        text: &str,
        singleline: bool,
        line: bool,
        blockinput: bool,
        r#type: &str,
    ) -> Self {
        #[derive(Serialize)]
        struct Args<'a> {
            text: &'a str,
            singleline: bool,
            line: bool,
            blockinput: bool,
            r#type: &'a str,
        }

        EditText(Widget::new(
            "EditText",
            parent,
            Args {
                text,
                singleline,
                line,
                blockinput,
                r#type,
            },
        ))
    }

    pub fn show_cursor(&self, show: bool) {
        #[derive(Serialize)]
        struct Args {
            show: bool,
        }

        self.send_msg("showCursor", Args { show });
    }
}

impl<'a> TextView<'a> for EditText<'a> {}

impl<'a> Deref for EditText<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
