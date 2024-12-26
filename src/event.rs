use serde::Deserialize;
use serde_json::Value;

pub const LONG_CLICK: &str = "longClick";
pub const FOCUS_CHANGE: &str = "focusChange";
pub const KEY: &str = "key";
pub const TOUCH: &str = "touch";
pub const REFRESH: &str = "refresh";
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

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "snake_case")]
pub enum Event {
    ScreenOn,
    ScreenOff,
    Click {
        id: i32,
        aid: i32,
        #[serde(default)]
        set: bool,
    },
    Text {
        id: i32,
        aid: i32,

        text: String,
    },
    Resume {
        aid: i32,
        finishing: bool,
    },
    Stop {
        aid: i32,
        finishing: bool,
    },
    Start {
        aid: i32,
        finishing: bool,
    },
    Create {
        aid: i32,
        finishing: bool,
    },
    Pause {
        aid: i32,
        finishing: bool,
    },
    Destroy {
        aid: i32,
        finishing: bool,
    },
    #[serde(untagged)]
    Other {
        #[serde(rename = "type")]
        ty: String,
        #[serde(default)]
        value: Option<Value>,
    },
}
