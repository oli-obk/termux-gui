use serde::Deserialize;
use serde_json::Value;

pub const CLICK: &str = "click";
pub const LONG_CLICK: &str = "longClick";
pub const FOCUS_CHANGE: &str = "focusChange";
pub const KEY: &str = "key";
pub const TOUCH: &str = "touch";
pub const REFRESH: &str = "refresh";
pub const SELECTED: &str = "selected";
pub const ITEM_SELECTED: &str = "itemselected";
pub const TEXT: &str = "text";

pub const CREATE: &str = "create";
pub const START: &str = "start";
pub const RESUME: &str = "resume";
pub const PAUSE: &str = "pause";
pub const STOP: &str = "stop";
pub const DESTROY: &str = "destroy";

pub const USER_LEAVE_HINT: &str = "UserLeaveHint";
pub const PIP_CHANGED: &str = "pipchanged";
pub const CONFIG: &str = "config";

pub const SCREEN_ON: &str = "screen_on";
pub const SCREEN_OFF: &str = "screen_off";
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
pub struct Event {
    #[serde(rename = "type")]
    pub ty: String,
    pub value: Value,
}

impl Event {
    pub fn aid(&self) -> Option<&str> {
        self.value.get("aid")?.as_str()
    }
    pub fn id(&self) -> Option<i32> {
        self.value.get("id")?.as_i64()?.try_into().ok()
    }
}
