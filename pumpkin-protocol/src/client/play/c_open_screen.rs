use pumpkin_util::text::TextComponent;

use pumpkin_macros::client_packet;
use serde::Serialize;

use crate::VarInt;

#[derive(Serialize)]
#[client_packet("play:open_screen")]
pub struct COpenScreen<'a> {
    window_id: VarInt,
    window_type: VarInt,
    window_title: &'a TextComponent,
}

impl<'a> COpenScreen<'a> {
    pub fn new(window_id: VarInt, window_type: VarInt, window_title: &'a TextComponent) -> Self {
        Self {
            window_id,
            window_type,
            window_title,
        }
    }
}
