use super::label::TextView;
use super::View;
use crate::activity::Activity;
use serde_json::json;

pub struct Button<'a> {
    id: i32,
    activity: &'a Activity<'a>,
}

impl<'a> Button<'a> {
    pub fn new(activity: &'a Activity<'a>, parent: Option<i32>, text: &str) -> Self {
        let mut args = json!({
            "text": text,
        });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }

        let id = activity.send_recv_msg("createButton", args);

        Button { id, activity }
    }
}

impl<'a> TextView for Button<'a> {}

impl<'a> View for Button<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> &Activity<'a> {
        self.activity
    }
}
