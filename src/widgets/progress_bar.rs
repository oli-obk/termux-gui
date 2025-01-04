use super::View;
use crate::activity::Activity;
use crate::layouts::Parent;
use serde_json::json;

#[derive(Copy, Clone)]
pub struct ProgressBar<'a> {
    id: i32,
    activity: Activity<'a>,
}

impl<'a> ProgressBar<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent) -> Self {
        let mut args = json!({});

        if let Some(id) = parent.id() {
            args["parent"] = json!(id);
        }

        let id = activity.send_recv_msg("createProgressBar", args);

        ProgressBar { id, activity }
    }

    pub fn set_progress(&self, progress: u8) {
        let args = json!({"progress": progress});
        self.send_msg("createProgressBar", args);
    }
}

impl<'a> View for ProgressBar<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> &Activity<'a> {
        &self.activity
    }
}
