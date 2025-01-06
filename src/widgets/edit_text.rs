use serde::Serialize;
use super::label::TextView;
use super::View;
use super::Widget;
use crate::activity::Activity;
use crate::layouts::Parent;
use serde_json::json;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct EditText<'a>(Widget<'a>);

impl<'a> EditText<'a> {
    pub fn new(
        activity: Activity<'a>,
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
            activity,
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
        let args = json!({
            "show": show
        });
        self.send_msg("showCursor", args);
    }
}

impl<'a> TextView<'a> for EditText<'a> {}

impl<'a> Deref for EditText<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
