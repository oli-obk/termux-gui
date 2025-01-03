use super::label::TextView;
use super::View;
use crate::activity::Activity;
use serde_json::json;

pub struct EditText<'a> {
    id: i32,
    activity: &'a Activity<'a>,
}

impl<'a> EditText<'a> {
    pub fn new(
        activity: &'a Activity<'a>,
        parent: Option<i32>,
        text: &str,
        single_line: bool,
        line: bool,
        block_input: bool,
        ty: &str,
    ) -> Self {
        let mut args = json!({
            "text": text,
            "singleline": single_line,
            "line": line,
            "blockinput": block_input,
            "type": ty
        });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }

        let id = activity.send_recv_msg("createEditText", args);

        EditText { id, activity }
    }

    pub fn show_cursor(&self, show: bool) {
        let args = json!({
            "show": show
        });
        self.send_msg("showCursor", args);
    }
}

impl<'a> TextView for EditText<'a> {}

impl<'a> View for EditText<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> &Activity<'a> {
        self.activity
    }
}
