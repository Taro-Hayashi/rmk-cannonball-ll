use rmk::types::action::{EncoderAction, KeyAction};
use rmk::{a, encoder, k, layer, user};

pub(crate) const COL: usize = 15;
pub(crate) const ROW: usize = 2;
pub(crate) const NUM_LAYER: usize = 2;
pub(crate) const NUM_ENCODER: usize = 3;
pub(crate) const SCROLL_LAYER: u8 = 1;

// USER key IDs (0..=7 are reserved by rmk for BLE profile control)
pub(crate) const USER_CPI_UP: u8 = 8;
pub(crate) const USER_CPI_DOWN: u8 = 9;
pub(crate) const USER_CURSOR_TOGGLE: u8 = 10;
pub(crate) const USER_SNIPE: u8 = 11;
pub(crate) const USER_SCROLL_SPEED_UP: u8 = 12;
pub(crate) const USER_SCROLL_SPEED_DOWN: u8 = 13;
pub(crate) const USER_SCROLL_INVERT_X: u8 = 14;
pub(crate) const USER_SCROLL_INVERT_Y: u8 = 15;
pub(crate) const USER_SAVE_SLOT_1: u8 = 16;
pub(crate) const USER_LOAD_SLOT_1: u8 = 17;
pub(crate) const USER_SAVE_SLOT_2: u8 = 18;
pub(crate) const USER_LOAD_SLOT_2: u8 = 19;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    #[cfg(feature = "sensor-rotated-180")]
    {
        return [
            layer!([
                [
                    k!(A), k!(B), k!(MouseBtn1), k!(C),
                    k!(E), k!(G), k!(H), k!(I),
                    k!(MouseBtn2), k!(J), k!(K), k!(L),
                    k!(Kc5), k!(Kc6), k!(Kc7)
                ],
                [
                    k!(N), k!(M), k!(P), k!(O),
                    k!(D), k!(F), k!(S), k!(Q),
                    k!(R), k!(T), k!(V), k!(U),
                    k!(Kc0), k!(Kc9), k!(Kc8)
                ]
            ]),
            layer!([
                [
                    a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent),
                    a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent),
                    a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent),
                    a!(Transparent), a!(Transparent), a!(Transparent)
                ],
                [
                    a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent),
                    a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent),
                    a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent),
                    a!(Transparent), a!(Transparent), a!(Transparent)
                ]
            ]),
        ];
    }

    #[cfg(not(feature = "sensor-rotated-180"))]
    {
    [
        layer!([
            [
                k!(MouseBtn2),   user!(8),       user!(9),     user!(10),
                user!(12),       user!(14),      k!(MouseBtn1), user!(15),
                k!(I),           k!(J),          k!(K),         k!(L),
                user!(16),       user!(17),      user!(18)
            ],
            [
                k!(N), k!(M), k!(P), k!(O),
                user!(11), user!(13), k!(S), k!(Q),
                k!(R), k!(T), k!(V), k!(U),
                k!(Kc0), k!(Kc9), user!(19)
            ]
        ]),
        layer!([
            [
                a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent),
                a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent),
                a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent),
                a!(Transparent), a!(Transparent), a!(Transparent)
            ],
            [
                a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent),
                a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent),
                a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent),
                a!(Transparent), a!(Transparent), a!(Transparent)
            ]
        ]),
    ]
    }
}

pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [
        [
            encoder!(user!(8), user!(9)),    // head: CPI Up / Down
            encoder!(user!(12), user!(13)),  // chest: Scroll Speed Up / Down
            encoder!(user!(14), user!(15)),  // leg: Scroll Invert X / Y
        ],
        [
            encoder!(a!(Transparent), a!(Transparent)),
            encoder!(a!(Transparent), a!(Transparent)),
            encoder!(a!(Transparent), a!(Transparent)),
        ],
    ]
}
