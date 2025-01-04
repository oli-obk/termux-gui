use super::activity::Activity;
use super::layouts::{
    frame_layout::FrameLayout, horizontal_scroll_view::HorizontalScrollView,
    linear_layout::LinearLayout, nested_scroll_view::NestedScrollView, radio_group::RadioGroup,
    swipe_refresh_layout::SwipeRefreshLayout, tab_layout::TabLayout, OneChildParent, Parent,
};
#[cfg(feature = "image")]
use super::widgets::image::ImageView;
use super::widgets::{
    button::Button, check_box::CheckBox, edit_text::EditText, label::Label,
    progress_bar::ProgressBar, radio_button::RadioButton, space::Space, spinner::Spinner,
    switch::Switch, toggle_button::ToggleButton,
};
#[cfg(feature = "web")]
use crate::widgets::web_view::WebView;

impl<'a> Activity<'a> {
    pub fn radio_button(self, parent: impl Parent, text: &str, check: bool) -> RadioButton<'a> {
        RadioButton::new(self, parent, text, check)
    }

    pub fn default_radio_button(self, parent: impl Parent, text: &str) -> RadioButton<'a> {
        self.radio_button(parent, text, false)
    }

    pub fn toggle_button(self, parent: impl Parent, text: &str, check: bool) -> ToggleButton<'a> {
        ToggleButton::new(self, parent, text, check)
    }

    pub fn default_toggle_button(self, parent: impl Parent, text: &str) -> ToggleButton<'a> {
        self.toggle_button(parent, text, false)
    }

    pub fn switch(self, parent: impl Parent, text: &str, check: bool) -> Switch<'a> {
        Switch::new(self, parent, text, check)
    }

    pub fn default_switch(self, parent: impl Parent, text: &str) -> Switch<'a> {
        self.switch(parent, text, false)
    }

    pub fn check_box(self, parent: impl Parent, text: &str, check: bool) -> CheckBox<'a> {
        CheckBox::new(self, parent, text, check)
    }

    pub fn default_check_box(self, parent: impl Parent, text: &str) -> CheckBox<'a> {
        self.check_box(parent, text, false)
    }

    pub fn label(
        self,
        parent: impl Parent,
        text: &str,
        selectable_text: bool,
        clickable_links: bool,
    ) -> Label<'a> {
        Label::new(self, parent, text, selectable_text, clickable_links)
    }

    pub fn default_label(self, parent: impl Parent, text: &str) -> Label<'a> {
        self.label(parent, text, false, false)
    }

    #[cfg(feature = "image")]
    pub fn image_view(self, parent: impl Parent) -> ImageView<'a> {
        ImageView::new(self, parent)
    }

    #[cfg(feature = "web")]
    pub fn web_view(self, parent: impl Parent) -> WebView<'a> {
        WebView::new(self, parent)
    }

    pub fn spinner(self, parent: impl Parent) -> Spinner<'a> {
        Spinner::new(self, parent)
    }

    pub fn space(self, parent: impl Parent) -> Space<'a> {
        Space::new(self, parent)
    }

    pub fn progress_bar(self, parent: impl Parent) -> ProgressBar<'a> {
        ProgressBar::new(self, parent)
    }

    pub fn button(self, parent: impl Parent, text: &str) -> Button<'a> {
        Button::new(self, parent, text)
    }

    pub fn edit_text(
        self,
        parent: impl Parent,
        text: &str,
        single_line: bool,
        line: bool,
        block_input: bool,
        ty: &str,
    ) -> EditText<'a> {
        EditText::new(self, parent, text, single_line, line, block_input, ty)
    }

    pub fn default_edit_text(self, parent: impl Parent, text: &str) -> EditText<'a> {
        self.edit_text(parent, text, false, true, false, "text")
    }

    pub fn linear_layout(self, parent: impl Parent, vertical: bool) -> LinearLayout<'a> {
        LinearLayout::new(self, parent, vertical)
    }

    pub fn swipe_refresh_layout(
        self,
        parent: impl Parent,
    ) -> (SwipeRefreshLayout<'a>, OneChildParent) {
        SwipeRefreshLayout::new(self, parent)
    }
    pub fn radio_group(self, parent: impl Parent) -> RadioGroup<'a> {
        RadioGroup::new(self, parent)
    }

    pub fn tab_layout(self, parent: impl Parent) -> TabLayout<'a> {
        TabLayout::new(self, parent)
    }

    pub fn frame_layout(self, parent: impl Parent) -> FrameLayout<'a> {
        FrameLayout::new(self, parent)
    }

    pub fn nested_scroll_view(
        self,
        parent: impl Parent,
        fill_viewport: bool,
        snapping: bool,
        no_bar: bool,
    ) -> (NestedScrollView<'a>, OneChildParent) {
        NestedScrollView::new(self, parent, fill_viewport, snapping, no_bar)
    }

    pub fn default_nested_scroll_view(
        self,
        parent: impl Parent,
    ) -> (NestedScrollView<'a>, OneChildParent) {
        self.nested_scroll_view(parent, false, false, false)
    }

    pub fn horizontal_scroll_view(
        self,
        parent: impl Parent,
        fill_viewport: bool,
        snapping: bool,
        no_bar: bool,
    ) -> (HorizontalScrollView<'a>, OneChildParent) {
        HorizontalScrollView::new(self, parent, fill_viewport, snapping, no_bar)
    }

    pub fn default_horizontal_scroll_view(
        self,
        parent: impl Parent,
    ) -> (HorizontalScrollView<'a>, OneChildParent) {
        self.horizontal_scroll_view(parent, false, false, false)
    }
}
