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
        single_line: bool,
        line: bool,
        block_input: bool,
        ty: &str,
    ) -> Self {
        let args = json!({
            "text": text,
            "singleline": single_line,
            "line": line,
            "blockinput": block_input,
            "type": ty
        });

        EditText(Widget::new(activity, "EditText", parent, args))
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
