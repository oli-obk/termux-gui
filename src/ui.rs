use super::activity::Activity;
use super::layouts::{
    frame_layout::FrameLayout, horizontal_scroll_view::HorizontalScrollView,
    linear_layout::LinearLayout, nested_scroll_view::NestedScrollView, radio_group::RadioGroup,
    swipe_refresh_layout::SwipeRefreshLayout, tab_layout::TabLayout, OneChildParent,
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

pub trait Parent<'a>: Sized {
    fn id(&self) -> Option<i32>;
    fn aid(&self) -> i32 {
        self.activity().aid()
    }
    fn activity(&self) -> Activity<'a>;

    fn radio_button(self, text: &str, check: bool) -> RadioButton<'a> {
        RadioButton::new(self, text, check)
    }

    fn default_radio_button(self, text: &str) -> RadioButton<'a> {
        self.radio_button(text, false)
    }

    fn toggle_button(self, text: &str, check: bool) -> ToggleButton<'a> {
        ToggleButton::new(self, text, check)
    }

    fn default_toggle_button(self, text: &str) -> ToggleButton<'a> {
        self.toggle_button(text, false)
    }

    fn switch(self, text: &str, check: bool) -> Switch<'a> {
        Switch::new(self, text, check)
    }

    fn default_switch(self, text: &str) -> Switch<'a> {
        self.switch(text, false)
    }

    fn check_box(self, text: &str, check: bool) -> CheckBox<'a> {
        CheckBox::new(self, text, check)
    }

    fn default_check_box(self, text: &str) -> CheckBox<'a> {
        self.check_box(text, false)
    }

    fn label(self, text: &str, selectable_text: bool, clickable_links: bool) -> Label<'a> {
        Label::new(self, text, selectable_text, clickable_links)
    }

    fn default_label(self, text: &str) -> Label<'a> {
        self.label(text, false, false)
    }

    #[cfg(feature = "image")]
    fn image_view(self) -> ImageView<'a> {
        ImageView::new(self)
    }

    #[cfg(feature = "web")]
    fn web_view(self) -> WebView<'a> {
        WebView::new(self)
    }

    fn spinner(self) -> Spinner<'a> {
        Spinner::new(self)
    }

    fn space(self) -> Space<'a> {
        Space::new(self)
    }

    fn progress_bar(self) -> ProgressBar<'a> {
        ProgressBar::new(self)
    }

    fn button(self, text: &str) -> Button<'a> {
        Button::new(self, text)
    }

    fn edit_text(
        self,

        text: &str,
        single_line: bool,
        line: bool,
        block_input: bool,
        ty: &str,
    ) -> EditText<'a> {
        EditText::new(self, text, single_line, line, block_input, ty)
    }

    fn default_edit_text(self, text: &str) -> EditText<'a> {
        self.edit_text(text, false, true, false, "text")
    }

    fn linear_layout(self, vertical: bool) -> LinearLayout<'a> {
        LinearLayout::new(self, vertical)
    }

    fn swipe_refresh_layout(self) -> (SwipeRefreshLayout<'a>, OneChildParent<'a>) {
        SwipeRefreshLayout::new(self)
    }
    fn radio_group(self) -> RadioGroup<'a> {
        RadioGroup::new(self)
    }

    fn tab_layout(self) -> TabLayout<'a> {
        TabLayout::new(self)
    }

    fn frame_layout(self) -> FrameLayout<'a> {
        FrameLayout::new(self)
    }

    fn nested_scroll_view(
        self,

        fill_viewport: bool,
        snapping: bool,
        no_bar: bool,
    ) -> (NestedScrollView<'a>, OneChildParent<'a>) {
        NestedScrollView::new(self, fill_viewport, snapping, no_bar)
    }

    fn default_nested_scroll_view(self) -> (NestedScrollView<'a>, OneChildParent<'a>) {
        self.nested_scroll_view(false, false, false)
    }

    fn horizontal_scroll_view(
        self,

        fill_viewport: bool,
        snapping: bool,
        no_bar: bool,
    ) -> (HorizontalScrollView<'a>, OneChildParent<'a>) {
        HorizontalScrollView::new(self, fill_viewport, snapping, no_bar)
    }

    fn default_horizontal_scroll_view(self) -> (HorizontalScrollView<'a>, OneChildParent<'a>) {
        self.horizontal_scroll_view(false, false, false)
    }
}
