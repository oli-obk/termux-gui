use super::label::TextView;
use super::View;
use super::Widget;
use crate::activity::Activity;
use crate::layouts::Parent;
use serde_json::json;

#[derive(Copy, Clone)]
pub struct Button<'a>(Widget<'a>);

impl<'a> Button<'a> {
    pub fn new(activity: Activity<'a>, parent: impl Parent, text: &str) -> Self {
        let args = json!({
            "text": text,
        });

        Button(Widget::new(activity, "Button", parent, &args))
    }
}

impl<'a> TextView<'a> for Button<'a> {}

impl<'a> View<'a> for Button<'a> {
    fn get_id(&self) -> i32 {
        self.0.id
    }

    fn get_activity(&self) -> Activity<'a> {
        self.0.activity
    }
}
