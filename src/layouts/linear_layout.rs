use super::{Parent, View, ViewGroup};
use crate::activity::Activity;
use serde_json::json;

#[derive(Copy, Clone)]
pub struct LinearLayout<'a> {
    activity: Activity<'a>,
    id: i32,
}

impl<'a> LinearLayout<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent, vertical: bool) -> Self {
        let mut args = json!({
            "vertical": vertical
        });

        if let Some(id) = parent.id() {
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
        &self.activity
    }
}

impl<'a> ViewGroup for LinearLayout<'a> {}

impl Parent for LinearLayout<'_> {
    fn id(&self) -> Option<i32> {
        Some(self.id)
    }
    fn aid(&self) -> i32 {
        self.activity.aid()
    }
}
