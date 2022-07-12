use super::connection::{construct_message, send_msg, send_recv_msg};
use super::{RawFd, View};
use serde_json::json;

pub mod linear_layout;

pub trait ViewGroup: View {
    fn clear_children(&self) {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id()
        });

        send_msg(self.get_sock(), construct_message("deleteChildren", &args));
    }
}
