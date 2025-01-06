use super::Vec2;
use super::{OneChildParent, Parent, View, ViewGroup};
use crate::activity::Activity;
use serde_json::json;

#[derive(Copy, Clone)]
pub struct HorizontalScrollView<'a> {
    activity: Activity<'a>,
    id: i32,
}

impl<'a> HorizontalScrollView<'a> {
    pub fn new(
        activity: Activity<'a>,
        parent: impl Parent<'a>,
        fill_viewport: bool,
        snapping: bool,
        no_bar: bool,
    ) -> (Self, OneChildParent) {
        let mut args =
            json!({ "fillviewport": fill_viewport, "snapping": snapping, "nobar": no_bar});

        if let Some(id) = parent.id() {
            args["parent"] = json!(id);
        }
        let id = activity.send_recv_msg("createHorizontalScrollView", args);

        (
            HorizontalScrollView { id, activity },
            OneChildParent {
                id,
                aid: activity.aid(),
            },
        )
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
}

impl<'a> View<'a> for HorizontalScrollView<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_activity(&self) -> Activity<'a> {
        self.activity
    }
}

impl<'a> ViewGroup<'a> for HorizontalScrollView<'a> {}
