use super::connection::{send_msg, send_recv_msg};
use super::utils::{Color, Vec2};
use super::RawFd;
use serde::Serialize;
use serde_json::json;

pub mod button;
pub mod check_box;
pub mod compound_button;
pub mod edit_text;
#[cfg(feature = "image")]
pub mod image;
pub mod label;
pub mod progress_bar;
pub mod radio_button;
pub mod space;
pub mod spinner;
pub mod switch;
pub mod toggle_button;

pub trait View {
    fn get_id(&self) -> i32;
    fn get_aid(&self) -> i32;
    fn get_sock(&self) -> &RawFd;

    fn send_recv_msg(&self, method: &str, params: impl Serialize) -> serde_json::Value
    where
        Self: Sized,
    {
        send_recv_msg(self.get_sock(), method, params)
    }
    fn send_msg(&self, method: &str, params: impl Serialize)
    where
        Self: Sized,
    {
        send_msg(self.get_sock(), method, params);
    }

    fn delete(&self)
    where
        Self: Sized,
    {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id()
        });
        self.send_msg("deleteView", args);
    }

    fn set_margin(&self, margin: i32, dir: Option<&str>)
    where
        Self: Sized,
    {
        let mut args = json!({
            "aid": self.get_aid(),
            "id": self.get_id(),
            "margin": margin
        });

        if let Some(val) = dir {
            args["dir"] = json!(val);
        }

        self.send_msg("setMargin", args);
    }

    fn set_width(&self, width: u16, px: bool)
    where
        Self: Sized,
    {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id(),
            "width": width,
            "px": px
        });

        self.send_msg("setWidth", args);
    }

    fn set_height(&self, height: u16, px: bool)
    where
        Self: Sized,
    {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id(),
            "height": height,
            "px": px
        });

        self.send_msg("setHeight", args);
    }

    fn set_dimensions(&self, dimensions: Vec2<u16>, px: bool)
    where
        Self: Sized,
    {
        self.set_width(dimensions.x, px);
        self.set_height(dimensions.y, px);
    }

    fn set_linear_layout_params(&self, weight: u16)
    where
        Self: Sized,
    {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id(),
            "weight": weight
        });

        self.send_msg("setLinearLayoutParams", args);
    }

    fn send_touch_event(&self, send: bool)
    where
        Self: Sized,
    {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id(),
            "send": send
        });

        self.send_msg("sendTouchEvent", args);
    }

    fn send_click_event(&self, send: bool)
    where
        Self: Sized,
    {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id(),
            "send": send
        });

        self.send_msg("sendClickEvent", args);
    }

    fn send_long_click_event(&self, send: bool)
    where
        Self: Sized,
    {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id(),
            "send": send
        });

        self.send_msg("sendLongClickEvent", args);
    }

    fn send_focus_change_event(&self, send: bool)
    where
        Self: Sized,
    {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id(),
            "send": send
        });

        self.send_msg("sendFocusChangeEvent", args);
    }

    fn get_dimensions(&self) -> Vec2<u16>
    where
        Self: Sized,
    {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id()
        });

        let ret = self.send_recv_msg("getDimensions", args);
        Vec2 {
            x: ret[0].to_string().parse().unwrap(),
            y: ret[1].to_string().parse().unwrap(),
        }
    }

    fn set_background_color(&self, color: Color)
    where
        Self: Sized,
    {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id(),
            "color": color.to_u32()
        });

        self.send_msg("setBackgroundColor", args);
    }

    fn set_visibility(&self, vis: u8)
    where
        Self: Sized,
    {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id(),
            "vis": vis
        });

        self.send_msg("setVisibility", args);
    }

    fn focus(&self, force_soft: bool)
    where
        Self: Sized,
    {
        let args = json!({
            "aid": self.get_aid(),
            "id": self.get_id(),
            "forceSoft": force_soft
        });

        self.send_msg("requestFocus", args);
    }
}
