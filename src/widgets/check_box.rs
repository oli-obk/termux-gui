use super::compound_button::CompoundButton;
use super::label::TextView;
use super::View;
use crate::activity::Activity;
use crate::layouts::Parent;
use serde_json::json;

#[derive(Copy, Clone)]
pub struct CheckBox<'a> {
    id: i32,
    activity: Activity<'a>,
}

impl<'a> CheckBox<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent, text: &str, check: bool) -> Self {
        let mut args = json!({
            "text": text,
            "checked": check
        });

        if let Some(id) = parent.id() {
            args["parent"] = json!(id);
        }

        let id = activity.send_recv_msg("createCheckBox", args);

        CheckBox { id, activity }
    }
}

impl<'a> TextView for CheckBox<'a> {}

impl<'a> CompoundButton for CheckBox<'a> {}

impl<'a> View for CheckBox<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> &Activity<'a> {
        &self.activity
    }
}
