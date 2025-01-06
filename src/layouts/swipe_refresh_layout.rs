use super::{OneChildParent, Parent, View, ViewGroup};
use crate::widgets::Widget;
use serde_json::json;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct SwipeRefreshLayout<'a>(Widget<'a>);

impl<'a> SwipeRefreshLayout<'a> {
    pub fn new(parent: impl Parent<'a>) -> (Self, OneChildParent<'a>) {
        let activity = parent.activity();
        let widget = SwipeRefreshLayout(Widget::new("SwipeRefreshLayout", parent, ()));
        (
            widget,
            OneChildParent {
                id: widget.get_id(),
                activity,
            },
        )
    }

    pub fn set_refreshing(&self, refresh: bool) {
        let args = json!({
            "refresh": refresh
        });
        self.send_msg("setRefreshing", args);
    }
}

impl<'a> Deref for SwipeRefreshLayout<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Widget<'a> {
        &self.0
    }
}

impl<'a> ViewGroup<'a> for SwipeRefreshLayout<'a> {}
