use super::label::TextView;
use serde::Serialize;

pub trait CompoundButton<'a>: TextView<'a> {
    fn set_checked(&mut self, checked: bool) {
        #[derive(Serialize)]
        struct Args {
            checked: bool,
        }
        self.send_msg("setChecked", Args { checked });
    }
}
