use rmk::types::action::{EncoderAction, KeyAction};
use rmk::{encoder, k, layer};

pub(crate) const COL: usize = 2;
pub(crate) const ROW: usize = 1;
pub(crate) const NUM_LAYER: usize = 1;
pub(crate) const NUM_ENCODER: usize = 3;
// SIZE = ROW * COL (for DirectPinMatrix)
pub(crate) const SIZE: usize = ROW * COL;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(A), k!(B)]
        ]),
    ]
}

pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [
        [
            encoder!(k!(Kb1), k!(Kb2)), // head
            encoder!(k!(Kb3), k!(Kb4)), // chest
            encoder!(k!(Kb5), k!(Kb6)), // leg
        ],
    ]
}
