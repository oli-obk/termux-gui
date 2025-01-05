use super::label::TextView;
use super::View;
use crate::activity::Activity;
use crate::layouts::Parent;
use serde_json::json;

#[derive(Copy, Clone)]
pub struct EditText<'a> {
    id: i32,
    activity: Activity<'a>,
}

impl<'a> EditText<'a> {
    pub fn new(
        activity: Activity<'a>,
        parent: impl Parent,
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

        if let Some(id) = parent.id() {
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

impl<'a> TextView<'a> for EditText<'a> {}

impl<'a> View<'a> for EditText<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> Activity<'a> {
        self.activity
    }
}
