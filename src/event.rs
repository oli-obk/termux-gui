use crate::utils::Vec2;
use serde::Deserialize;

mod handler;

pub use handler::Handler;

pub const KEY: &str = "key";
pub const ITEM_SELECTED: &str = "itemselected";

pub const OVERLAY_TOUCH: &str = "overlay_touch";
pub const OVERLAY_SCALE: &str = "overlay_scale";

/// Events from the OS, not connected to the Activity or the App at all.
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum System {
    ScreenOn,
    ScreenOff,
    Airplane {
        value: bool,
    },
    Locale {
        ///The language code as a ISO 639 string.
        value: String,
    },
    Timezone,
    Config,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum App {
    Pipchanged {
        /// Whether the Activity is now in pip mode or not.
        value: bool,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Activity {
    Resume,
    Start,
    Stop {
        finishing: bool,
    },
    Destroy {
        finishing: bool,
    },
    Create,
    Pause {
        finishing: bool,
    },
    /// Send when you set an Activity to intercept the back button press and the back button is pressed.
    Back,
    /// Gets fired when the user leaves an Activity. Can be used >
    #[serde(rename = "UserLeaveHint")]
    UserLeaveHint,
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
    /// Send when the Text of the View changed, even when the text was changed with setText.
    Text {
        text: String,
    },
    Refresh,
    FocusChange {
        /// whether or not the View now has focus.
        focus: bool,
    },
    Touch {
        #[serde(flatten)]
        action: TouchAction,
        ///  The time of the event in milliseconds since boot excluding sleep. Use this when checking for gestures or other time-sensitive things.
        time: u64,
        pointers: Vec<TouchPoint>,
    },
    /// A RadioButton in a RadioButtonGroup has been selected
    Selected {
        /// The id of the now selected RadioButton
        selected: i32,
    },
}

#[derive(Debug, Deserialize)]
pub struct TouchPoint {
    /// The coordinates of the pointer inside the View (not in the window).
    /// For ImageView, these are the coordinates of the pixel in the displayed image or buffer,
    /// so you don't need to convert the position yourself.
    #[serde(flatten)]
    pub pos: Vec2<u32>,
    /// The pointer id.
    ///This stays consistent for each pointer in the frame between "up" and "down" events
    pub id: u32,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum TouchAction {
    Up,
    Down,
    PointerUp { index: u32 },
    PointerDown { index: u32 },
    Cancel,
    Move,
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
        #[serde(flatten)]
        kind: Activity,
    },
    System(System),
    App(App),
}
