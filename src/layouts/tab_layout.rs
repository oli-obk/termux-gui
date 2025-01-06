use super::Vec2;
use super::{Parent, View, ViewGroup};
use crate::activity::Activity;
use crate::widgets::Widget;
use serde_json::json;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct TabLayout<'a>(Widget<'a>);

impl<'a> TabLayout<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent<'a>) -> Self {
        TabLayout(Widget::new(activity, "TabLayout", parent, ()))
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

impl<'a> Deref for TabLayout<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> ViewGroup<'a> for TabLayout<'a> {}

impl<'a, T: Deref<Target = Widget<'a>> + ViewGroup<'a>> Parent<'a> for T {
    fn id(&self) -> Option<i32> {
        Some(self.get_id())
    }
    fn aid(&self) -> i32 {
        self.get_activity().aid()
    }
}
