use super::View;
use crate::activity::Activity;
use crate::layouts::Parent;
use serde_json::json;

#[derive(Copy, Clone)]
pub struct Space<'a> {
    id: i32,
    activity: Activity<'a>,
}

impl<'a> Space<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent) -> Self {
        let mut args = json!({});

        if let Some(id) = parent.id() {
            args["parent"] = json!(id);
        }

        let id = activity.send_recv_msg("createSpace", args);

        Space { id, activity }
    }
}

impl<'a> View for Space<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> Activity<'a> {
        self.activity
    }
}
