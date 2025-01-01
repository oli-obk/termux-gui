use serde::Deserialize;
use serde_json::{Map, Value};

mod handler;

pub use handler::Handler;

pub const FOCUS_CHANGE: &str = "focusChange";
pub const KEY: &str = "key";
pub const TOUCH: &str = "touch";
pub const SELECTED: &str = "selected";
pub const ITEM_SELECTED: &str = "itemselected";

pub const USER_LEAVE_HINT: &str = "UserLeaveHint";
pub const PIP_CHANGED: &str = "pipchanged";
pub const CONFIG: &str = "config";

pub const TIMEZONE: &str = "timezone";
pub const LOCALE: &str = "locale";
pub const AIRPLANE: &str = "airplane";

pub const OVERLAY_TOUCH: &str = "overlay_touch";
pub const OVERLAY_SCALE: &str = "overlay_scale";

pub const TOUCH_UP: &str = "up";
pub const TOUCH_DOWN: &str = "down";
pub const TOUCH_POINTER_UP: &str = "pointer_up";
pub const TOUCH_POINTER_DOWN: &str = "pointer_down";
pub const TOUCH_CANCEL: &str = "cancel";
pub const TOUCH_MOVE: &str = "move";

/// Events from the OS, not connected to the Activity or the App at all.
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum System {
    ScreenOn,
    ScreenOff,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Activity {
    Resume,
    Start,
    Stop,
    Destroy,
    Create,
    Pause,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Widget {
    Click {
        #[serde(default)]
        set: bool,
    },
    LongClick,
    Text {
        text: String,
    },
    Refresh,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    Widget {
        id: i32,
        aid: i32,
        #[serde(flatten)]
        kind: Widget,
    },
    Activity {
        aid: i32,
        finishing: bool,
        #[serde(rename = "type")]
        kind: Activity,
    },
    System(System),
    Other {
        #[serde(rename = "type")]
        ty: String,
        #[serde(flatten)]
        value: Map<String, Value>,
    },
}
