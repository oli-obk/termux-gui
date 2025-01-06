use super::{Parent, ViewGroup};
use crate::widgets::Widget;
use serde_json::json;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct LinearLayout<'a>(Widget<'a>);

impl<'a> LinearLayout<'a> {
    pub fn new(parent: impl Parent<'a>, vertical: bool) -> Self {
        let args = json!({
            "vertical": vertical
        });

        LinearLayout(Widget::new("LinearLayout", parent, args))
    }
}

impl<'a> ViewGroup<'a> for LinearLayout<'a> {}

impl<'a> Deref for LinearLayout<'a> {
    type Target = Widget<'a>;
    fn deref(&self) -> &Widget<'a> {
        &self.0
    }
}
