use rmk::types::action::{EncoderAction, KeyAction};
use rmk::{encoder, k, layer};

pub(crate) const COL: usize = 16;
pub(crate) const ROW: usize = 2;
pub(crate) const NUM_LAYER: usize = 1;
pub(crate) const NUM_ENCODER: usize = 3;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            // Test keymap — each key outputs a unique character for ghosting diagnosis
            //  Col:    0       1       2       3       4       5       6       7       8       9       10      11      12      13      14         15
            // ZMK:   Key     Key     Key     Key     Key     Key     Key     Key     Key     Key     Mouse   Mouse   Side    Side    Side       (none)
            [k!(A),  k!(B),  k!(C),  k!(D),  k!(E),  k!(F),  k!(G),  k!(H),  k!(I),  k!(J),  k!(K),  k!(L),  k!(M),  k!(N),  k!(Kc1), k!(No)],
            // ZMK:   Stick   Stick   Stick   Stick   Key     Key     Lever   Lever   Lever   Lever   Lever   Lever   Back    Back    Side       Center
            [k!(O),  k!(P),  k!(Q),  k!(R),  k!(S),  k!(T),  k!(U),  k!(V),  k!(W),  k!(X),  k!(Y),  k!(Z),  k!(Kc3), k!(Kc4), k!(Kc5), k!(Kc6)]
        ]),
    ]
}

pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [
        [
            encoder!(k!(Kc7), k!(Kc8)), // head
            encoder!(k!(Kc9), k!(Kc0)), // chest
            encoder!(k!(Minus), k!(Equal)), // leg
        ],
    ]
}
