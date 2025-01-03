use super::activity::{Activity, InputMode};
use super::layouts::{
    frame_layout::FrameLayout, horizontal_scroll_view::HorizontalScrollView,
    linear_layout::LinearLayout, nested_scroll_view::NestedScrollView, radio_group::RadioGroup,
    swipe_refresh_layout::SwipeRefreshLayout, tab_layout::TabLayout,
};
#[cfg(feature = "image")]
use super::widgets::image::ImageView;
use super::widgets::{
    button::Button, check_box::CheckBox, edit_text::EditText, label::Label,
    progress_bar::ProgressBar, radio_button::RadioButton, space::Space, spinner::Spinner,
    switch::Switch, toggle_button::ToggleButton, View,
};
use crate::TGui;

pub struct Ui<'a> {
    activity: Activity<'a>,
}

impl<'a> Ui<'a> {
    pub fn new(gui: &'a TGui, flags: crate::activity::Flags) -> Self {
        Ui {
            activity: Activity::new(gui, flags),
        }
    }

    pub fn set_input_mode(&self, mode: InputMode) {
        self.activity.set_input_mode(mode)
    }

    pub fn radio_button(&self, parent: Option<&dyn View>, text: &str, check: bool) -> RadioButton {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        RadioButton::new(&self.activity, parent, text, check)
    }

    pub fn default_radio_button(&self, parent: Option<&dyn View>, text: &str) -> RadioButton {
        self.radio_button(parent, text, false)
    }

    pub fn toggle_button(
        &self,
        parent: Option<&dyn View>,
        text: &str,
        check: bool,
    ) -> ToggleButton {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        ToggleButton::new(&self.activity, parent, text, check)
    }

    pub fn default_toggle_button(&self, parent: Option<&dyn View>, text: &str) -> ToggleButton {
        self.toggle_button(parent, text, false)
    }

    pub fn switch(&self, parent: Option<&dyn View>, text: &str, check: bool) -> Switch {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        Switch::new(&self.activity, parent, text, check)
    }

    pub fn default_switch(&self, parent: Option<&dyn View>, text: &str) -> Switch {
        self.switch(parent, text, false)
    }

    pub fn check_box(&self, parent: Option<&dyn View>, text: &str, check: bool) -> CheckBox {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        CheckBox::new(&self.activity, parent, text, check)
    }

    pub fn default_check_box(&self, parent: Option<&dyn View>, text: &str) -> CheckBox {
        self.check_box(parent, text, false)
    }

    pub fn label(
        &self,
        parent: Option<&dyn View>,
        text: &str,
        selectable_text: bool,
        clickable_links: bool,
    ) -> Label {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        Label::new(
            &self.activity,
            parent,
            text,
            selectable_text,
            clickable_links,
        )
    }

    pub fn default_label(&self, parent: Option<&dyn View>, text: &str) -> Label {
        self.label(parent, text, false, false)
    }

    #[cfg(feature = "image")]
    pub fn image_view(&self, parent: Option<&dyn View>) -> ImageView {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        ImageView::new(&self.activity, parent)
    }

    pub fn spinner(&self, parent: Option<&dyn View>) -> Spinner {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        Spinner::new(&self.activity, parent)
    }

    pub fn space(&self, parent: Option<&dyn View>) -> Space {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        Space::new(&self.activity, parent)
    }

    pub fn progress_bar(&self, parent: Option<&dyn View>) -> ProgressBar {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        ProgressBar::new(&self.activity, parent)
    }

    pub fn button(&self, parent: Option<&dyn View>, text: &str) -> Button {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        Button::new(&self.activity, parent, text)
    }

    pub fn edit_text(
        &self,
        parent: Option<&dyn View>,
        text: &str,
        single_line: bool,
        line: bool,
        block_input: bool,
        ty: &str,
    ) -> EditText {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        EditText::new(
            &self.activity,
            parent,
            text,
            single_line,
            line,
            block_input,
            ty,
        )
    }

    pub fn default_edit_text(&self, parent: Option<&dyn View>, text: &str) -> EditText {
        self.edit_text(parent, text, false, true, false, "text")
    }

    pub fn linear_layout(&self, parent: Option<&dyn View>, vertical: bool) -> LinearLayout {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        LinearLayout::new(&self.activity, parent, vertical)
    }

    pub fn swipe_refresh_layout(&self, parent: Option<&dyn View>) -> SwipeRefreshLayout {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        SwipeRefreshLayout::new(&self.activity, parent)
    }
    pub fn radio_group(&self, parent: Option<&dyn View>) -> RadioGroup {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        RadioGroup::new(&self.activity, parent)
    }

    pub fn tab_layout(&self, parent: Option<&dyn View>) -> TabLayout {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        TabLayout::new(&self.activity, parent)
    }

    pub fn frame_layout(&self, parent: Option<&dyn View>) -> FrameLayout {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        FrameLayout::new(&self.activity, parent)
    }

    pub fn nested_scroll_view(
        &self,
        parent: Option<&dyn View>,
        fill_viewport: bool,
        snapping: bool,
        no_bar: bool,
    ) -> NestedScrollView {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        NestedScrollView::new(&self.activity, parent, fill_viewport, snapping, no_bar)
    }

    pub fn default_nested_scroll_view(&self, parent: Option<&dyn View>) -> NestedScrollView {
        self.nested_scroll_view(parent, false, false, false)
    }

    pub fn horizontal_scroll_view(
        &self,
        parent: Option<&dyn View>,
        fill_viewport: bool,
        snapping: bool,
        no_bar: bool,
    ) -> HorizontalScrollView {
        let parent: Option<i32> = match parent {
            Some(parent) => Some(parent.get_id()),
            None => None,
        };
        HorizontalScrollView::new(&self.activity, parent, fill_viewport, snapping, no_bar)
    }

    pub fn default_horizontal_scroll_view(
        &self,
        parent: Option<&dyn View>,
    ) -> HorizontalScrollView {
        self.horizontal_scroll_view(parent, false, false, false)
    }

    pub fn finish(&self) {
        self.activity.finish();
    }
}
