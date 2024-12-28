use super::View;
use crate::activity::Activity;
use serde_json::json;

pub struct Space<'a> {
    id: i32,
    activity: &'a Activity<'a>,
}

impl<'a> Space<'a> {
    pub fn new(activity: &'a Activity<'a>, parent: Option<i32>) -> Self {
        let mut args = json!({});

        if let Some(id) = parent {
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

    fn get_activity(&self) -> &Activity<'a> {
        self.activity
    }
}
