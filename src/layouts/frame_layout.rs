use super::{Parent, View, ViewGroup};
use crate::activity::Activity;
use serde_json::json;

#[derive(Copy, Clone)]
pub struct FrameLayout<'a> {
    activity: Activity<'a>,
    id: i32,
}

impl<'a> FrameLayout<'a> {
    #[must_use]
    pub fn new(activity: Activity<'a>, parent: impl Parent<'a>) -> Self {
        let mut args = json!({});

        if let Some(id) = parent.id() {
            args["parent"] = json!(id);
        }
        assert_eq!(parent.aid(), activity.aid());
        let id = activity.send_recv_msg("createFrameLayout", args);

        FrameLayout { id, activity }
    }
}

impl<'a> View<'a> for FrameLayout<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> Activity<'a> {
        self.activity
    }
}

impl<'a> ViewGroup<'a> for FrameLayout<'a> {}

impl Parent<'_> for FrameLayout<'_> {
    fn id(&self) -> Option<i32> {
        Some(self.id)
    }
    fn aid(&self) -> i32 {
        self.activity.aid()
    }
}
