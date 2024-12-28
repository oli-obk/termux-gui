use super::utils::Vec2;
use super::View;

pub mod frame_layout;
pub mod horizontal_scroll_view;
pub mod linear_layout;
pub mod nested_scroll_view;
pub mod radio_group;
pub mod swipe_refresh_layout;
pub mod tab_layout;

pub trait ViewGroup: View + Sized {
    fn clear_children(&self) {
        self.send_msg("deleteChildren", ());
    }
}
