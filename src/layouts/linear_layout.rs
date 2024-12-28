use super::{View, ViewGroup};
use crate::activity::Activity;
use serde_json::json;

pub struct LinearLayout<'a> {
    activity: &'a Activity<'a>,
    id: i32,
}

impl<'a> LinearLayout<'a> {
    pub fn new(activity: &'a Activity<'a>, parent: Option<i32>, vertical: bool) -> Self {
        let mut args = json!({
            "vertical": vertical
        });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }
        let id = activity.send_recv_msg("createLinearLayout", args);

        LinearLayout { id, activity }
    }
}

impl<'a> View for LinearLayout<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> &Activity<'a> {
        self.activity
    }
}

impl<'a> ViewGroup for LinearLayout<'a> {}
