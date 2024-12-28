use super::Vec2;
use super::{View, ViewGroup};
use crate::activity::Activity;
use serde_json::json;

pub struct NestedScrollView<'a> {
    activity: &'a Activity<'a>,
    id: i32,
}

impl<'a> NestedScrollView<'a> {
    pub fn new(
        activity: &'a Activity<'a>,
        parent: Option<i32>,
        fill_viewport: bool,
        snapping: bool,
        no_bar: bool,
    ) -> Self {
        let mut args =
            json!({ "fillviewport": fill_viewport, "snapping": snapping, "nobar": no_bar});

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }
        let id = activity.send_recv_msg("createNestedScrollView", args);

        NestedScrollView { id, activity }
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
        let ret = self.send_recv_msg("getScrollPosition", ());
        let x: u16 = ret["x"].to_string().parse().unwrap();
        let y: u16 = ret["y"].to_string().parse().unwrap();
        Vec2 { x, y }
    }
}

impl<'a> View for NestedScrollView<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> &Activity<'a> {
        self.activity
    }
}

impl<'a> ViewGroup for NestedScrollView<'a> {}
