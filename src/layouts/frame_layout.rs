use super::{View, ViewGroup};
use crate::activity::Activity;
use serde_json::json;

pub struct FrameLayout<'a> {
    activity: &'a Activity<'a>,
    id: i32,
}

impl<'a> FrameLayout<'a> {
    pub fn new(activity: &'a Activity<'a>, parent: Option<i32>) -> Self {
        let mut args = json!({});

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }
        let id = activity.send_recv_msg("createFrameLayout", args);

        FrameLayout { id, activity }
    }
}

impl<'a> View for FrameLayout<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> &Activity<'a> {
        self.activity
    }
}

impl<'a> ViewGroup for FrameLayout<'a> {}
