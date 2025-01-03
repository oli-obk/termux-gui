use super::View;
use crate::activity::Activity;
use serde_json::json;

pub struct Spinner<'a> {
    id: i32,
    activity: &'a Activity<'a>,
}

impl<'a> Spinner<'a> {
    pub fn new(activity: &'a Activity<'a>, parent: Option<i32>) -> Self {
        let mut args = json!({});

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }
        let id = activity.send_recv_msg("createSpinner", &args);

        Spinner { id, activity }
    }

    pub fn set_list(&self, list: &[&str]) {
        let args = json!({
            "list": list
        });

        self.send_msg("setList", args);
    }
}

impl<'a> View for Spinner<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> &Activity<'a> {
        self.activity
    }
}
