use super::{View, ViewGroup};
use crate::activity::Activity;
use serde_json::json;

pub struct SwipeRefreshLayout<'a> {
    activity: Activity<'a>,
    id: i32,
}

impl<'a> SwipeRefreshLayout<'a> {
    pub fn new(activity: Activity<'a>, parent: Option<i32>) -> Self {
        let mut args = json!({});

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }
        let id = activity.send_recv_msg("createSwipeRefreshLayout", args);

        SwipeRefreshLayout { id, activity }
    }

    pub fn set_refreshing(&self, refresh: bool) {
        let args = json!({
            "refresh": refresh
        });
        self.send_msg("setRefreshing", args);
    }
}

impl<'a> View for SwipeRefreshLayout<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> &Activity<'a> {
        &self.activity
    }
}

impl<'a> ViewGroup for SwipeRefreshLayout<'a> {}
