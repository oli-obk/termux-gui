use serde_json::json;
use std::os::unix::net::UnixStream;

type RawFd = UnixStream;

pub mod connection;

pub mod activity;
pub mod event;
pub mod layouts;
pub mod ui;
pub mod utils;
pub mod widgets;

use widgets::View;

pub struct TGui {
    pub main: UnixStream,
    pub event: UnixStream,
}

impl TGui {
    pub fn new() -> Self {
        let (main, event) = connection::connect();
        TGui { main, event }
    }

    pub fn activity(&self, flags: activity::Flags) -> activity::Activity {
        activity::Activity::new(&self.main, flags)
    }

    pub fn ui(&self, flags: activity::Flags) -> ui::Ui {
        ui::Ui::new(&self.main, flags)
    }

    pub fn event(&self) -> Result<event::Event, serde_json::Error> {
        connection::try_recv_msg(&self.event)
    }

    pub fn toast(&self, text: &str, long: bool) {
        let args = json!({
            "text": text,
            "long": long
        });
        connection::send_msg(&self.main, "toast", args);
    }

    pub fn turn_screen_on(&self) {
        connection::send_msg(&self.main, "turnScreenOn", ());
    }

    pub fn is_locked(&self) -> bool {
        connection::send_recv_msg(&self.main, "isLocked", ())
    }
}
