use serde_json::json;
use std::os::unix::net::UnixStream;

pub mod activity;
pub mod connection;
pub mod event;
pub mod layouts;
pub mod ui;
pub mod utils;
pub mod widgets;

use widgets::View;

pub struct TGui {
    main: UnixStream,
    event: UnixStream,
    /// Whether to dump stream messages to stderr
    pub debug: bool,
}

impl TGui {
    pub fn new() -> Self {
        let (main, event) = connection::connect();
        TGui {
            main,
            event,
            debug: false,
        }
    }

    pub fn new_activity<'a>(&'a self, flags: activity::Flags) -> activity::Activity<'a> {
        activity::Activity::new(self, flags)
    }

    pub fn event(&self) -> Result<event::Event, serde_json::Error> {
        connection::try_recv_msg(&self.event)
    }

    pub fn toast(&self, text: &str, long: bool) {
        let args = json!({
            "text": text,
            "long": long
        });
        self.send_msg("toast", args);
    }

    pub fn turn_screen_on(&self) {
        self.send_msg("turnScreenOn", ());
    }

    pub fn is_locked(&self) -> bool {
        self.send_recv_msg("isLocked", ())
    }
}
