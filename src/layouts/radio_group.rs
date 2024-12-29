use super::{View, ViewGroup};
use crate::activity::Activity;
use serde_json::json;

pub struct RadioGroup<'a> {
    activity: Activity<'a>,
    id: i32,
}

impl<'a> RadioGroup<'a> {
    pub fn new(activity: Activity<'a>, parent: Option<i32>) -> Self {
        let mut args = json!({});

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }
        let id = activity.send_recv_msg("createRadioGroup", args);

        RadioGroup { id, activity }
    }
}

impl<'a> View for RadioGroup<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> &Activity<'a> {
        &self.activity
    }
}

impl<'a> ViewGroup for RadioGroup<'a> {}
