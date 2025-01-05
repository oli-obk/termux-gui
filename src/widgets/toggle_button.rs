use super::compound_button::CompoundButton;
use super::label::TextView;
use super::View;
use crate::activity::Activity;
use crate::layouts::Parent;
use serde_json::json;

#[derive(Copy, Clone)]
pub struct ToggleButton<'a> {
    id: i32,
    activity: Activity<'a>,
}

impl<'a> ToggleButton<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent, text: &str, check: bool) -> Self {
        let mut args = json!({
            "text": text,
            "checked": check
        });

        if let Some(id) = parent.id() {
            args["parent"] = json!(id);
        }

        let id = activity.send_recv_msg("createToggleButton", args);
        ToggleButton { id, activity }
    }
}

impl<'a> TextView<'a> for ToggleButton<'a> {}

impl<'a> CompoundButton<'a> for ToggleButton<'a> {}

impl<'a> View<'a> for ToggleButton<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> Activity<'a> {
        self.activity
    }
}
