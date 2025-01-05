use super::label::TextView;
use serde_json::json;

pub trait CompoundButton: TextView {
    fn set_checked(&mut self, set: bool) {
        let args = json!({
            "checked": set
        });
        self.send_msg("setChecked", args);
    }
}
