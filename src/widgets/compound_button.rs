use super::label::TextView;
use crate::event::{self, Event};
use serde_json::json;

pub trait CompoundButton: TextView {
    fn check(&mut self, set: bool);
    fn handle_event(&mut self, e: &Event) {
        if let &Event::Widget {
            aid,
            id,
            kind: event::Widget::Click { set },
        } = e
        {
            if aid == self.get_aid() && id == self.get_id() {
                self.check(set);
            }
        }
    }

    fn set_checked(&mut self, set: bool) {
        self.check(set);
        let args = json!({
            "checked": set
        });
        self.send_msg("setChecked", args);
    }
}
