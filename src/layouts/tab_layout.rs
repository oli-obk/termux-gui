use super::Vec2;
use super::{Parent, View, ViewGroup};
use crate::activity::Activity;
use serde_json::json;

#[derive(Copy, Clone)]
pub struct TabLayout<'a> {
    activity: Activity<'a>,
    id: i32,
}

impl<'a> TabLayout<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent<'a>) -> Self {
        let mut args = json!({});

        if let Some(id) = parent.id() {
            args["parent"] = json!(id);
        }
        let id = activity.send_recv_msg("createTabLayout", args);

        TabLayout { id, activity }
    }

    pub fn set_scroll_position(&self, pos: Vec2<u16>, smooth: bool) {
        let args = json!({
           "x": pos.x,
           "y": pos.y,
           "soft": smooth
        });
        self.send_msg("setScrollPosition", args);
    }

    pub fn get_scroll_position(&self) -> Vec2<u16> {
        self.send_recv_msg("getScrollPosition", ())
    }

    pub fn set_list(&self, list: &[&str]) {
        let args = json!({
            "list": list
        });

        self.send_msg("setList", args);
    }
}

impl<'a> View<'a> for TabLayout<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> Activity<'a> {
        self.activity
    }
}

impl<'a> ViewGroup<'a> for TabLayout<'a> {}

impl Parent<'_> for TabLayout<'_> {
    fn id(&self) -> Option<i32> {
        Some(self.id)
    }
    fn aid(&self) -> i32 {
        self.activity.aid()
    }
}
