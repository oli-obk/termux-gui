use super::utils::Vec2;
use super::View;

pub mod frame_layout;
pub mod horizontal_scroll_view;
pub mod linear_layout;
pub mod nested_scroll_view;
pub mod radio_group;
pub mod swipe_refresh_layout;
pub mod tab_layout;

pub trait ViewGroup<'a>: View<'a> + Sized + Copy {
    fn clear_children(&self) -> OneChildParent {
        self.send_msg("deleteChildren", ());
        OneChildParent {
            id: self.get_id(),
            aid: self.get_activity().aid(),
        }
    }
}

pub trait Parent {
    fn id(&self) -> Option<i32>;
    fn aid(&self) -> i32;
}

/// A layout that can only have one child needs this
/// token for registering said child
pub struct OneChildParent {
    aid: i32,
    id: i32,
}

impl Parent for OneChildParent {
    fn id(&self) -> Option<i32> {
        Some(self.id)
    }
    fn aid(&self) -> i32 {
        self.aid
    }
}

impl Parent for crate::activity::Activity<'_> {
    fn id(&self) -> Option<i32> {
        None
    }
    fn aid(&self) -> i32 {
        self.aid()
    }
}
