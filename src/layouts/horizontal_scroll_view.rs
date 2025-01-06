use super::Vec2;
use super::{OneChildParent, Parent, View, ViewGroup};
use crate::activity::Activity;
use crate::widgets::Widget;
use serde_json::json;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct HorizontalScrollView<'a>(Widget<'a>);

impl<'a> HorizontalScrollView<'a> {
    pub fn new(
        activity: Activity<'a>,
        parent: impl Parent<'a>,
        fill_viewport: bool,
        snapping: bool,
        no_bar: bool,
    ) -> (Self, OneChildParent) {
        let args =
            json!({ "fillviewport": fill_viewport, "snapping": snapping, "nobar": no_bar});
        let widget =
            HorizontalScrollView(Widget::new(activity, "HorizontalScrollView", parent, args));
        (
            widget,
            OneChildParent {
                id: widget.get_id(),
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

impl<'a> Deref for HorizontalScrollView<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Widget<'a> {
        &self.0
    }
}

impl<'a> ViewGroup<'a> for HorizontalScrollView<'a> {}
