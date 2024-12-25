use super::Event;
use super::{construct_message, label::TextView};
use serde_json::json;

pub trait CompoundButton: TextView {
    fn check(&mut self, set: bool);
    fn handle_event(&mut self, e: &Event) {
        if let &Event::Click { aid, id, set } = e {
            if aid == self.get_aid() && id == self.get_id() {
                self.check(set);
            }
        }
    }

    fn set_checked(&mut self, set: bool) {
        self.check(set);
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id(),
            "checked": set
        });
        self.send_msg(construct_message("setChecked", &args));
    }
}
