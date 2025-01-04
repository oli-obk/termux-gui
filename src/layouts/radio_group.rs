use super::{Parent, View, ViewGroup};
use crate::activity::Activity;
use serde_json::json;

#[derive(Copy, Clone)]
pub struct RadioGroup<'a> {
    activity: Activity<'a>,
    id: i32,
}

impl<'a> RadioGroup<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent) -> Self {
        let mut args = json!({});

        if let Some(id) = parent.id() {
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

impl Parent for RadioGroup<'_> {
    fn id(&self) -> Option<i32> {
        Some(self.id)
    }
    fn aid(&self) -> i32 {
        self.activity.aid()
    }
}
