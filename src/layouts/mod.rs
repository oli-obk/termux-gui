use super::utils::Vec2;
use super::View;
use crate::activity::Activity;
use crate::widgets::Widget;
use std::ops::Deref;

pub mod frame_layout;
pub mod horizontal_scroll_view;
pub mod linear_layout;
pub mod nested_scroll_view;
pub mod radio_group;
pub mod swipe_refresh_layout;
pub mod tab_layout;

pub trait ViewGroup<'a>: View<'a> + Sized + Copy {
    fn clear_children(&self) -> OneChildParent<'a> {
        self.send_msg("deleteChildren", ());
        OneChildParent {
            id: self.get_id(),
            activity: self.get_activity(),
        }
    }
}

/// A layout that can only have one child needs this
/// token for registering said child
pub struct OneChildParent<'a> {
    activity: Activity<'a>,
    id: i32,
}

pub use crate::ui::Parent;

impl<'a> Parent<'a> for OneChildParent<'a> {
    fn id(&self) -> Option<i32> {
        Some(self.id)
    }
    fn activity(&self) -> Activity<'a> {
        self.activity
    }
}

impl<'a> Parent<'a> for Activity<'a> {
    fn id(&self) -> Option<i32> {
        None
    }
    fn activity(&self) -> Activity<'a> {
        *self
    }
}

impl<'a, T: Deref<Target = Widget<'a>> + ViewGroup<'a>> Parent<'a> for T {
    fn id(&self) -> Option<i32> {
        Some(self.get_id())
    }
    fn activity(&self) -> Activity<'a> {
        self.get_activity()
    }
}
