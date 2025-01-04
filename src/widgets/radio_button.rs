use super::compound_button::CompoundButton;
use super::label::TextView;
use super::View;
use crate::activity::Activity;
use crate::layouts::Parent;
use serde_json::json;

pub struct RadioButton<'a> {
    id: i32,
    activity: Activity<'a>,
    check: bool,
}

impl<'a> RadioButton<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent, text: &str, check: bool) -> Self {
        let mut args = json!({
            "text": text,
            "checked": check
        });

        if let Some(id) = parent.id() {
            args["parent"] = json!(id);
        }

        let id = activity.send_recv_msg("createRadioButton", args);

        RadioButton {
            id,
            activity,
            check,
        }
    }
}

impl<'a> TextView for RadioButton<'a> {}

impl<'a> CompoundButton for RadioButton<'a> {
    fn check(&mut self, set: bool) {
        self.check = set;
    }
}

impl<'a> View for RadioButton<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> &Activity<'a> {
        &self.activity
    }
}
