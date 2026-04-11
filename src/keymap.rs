use rmk::types::action::{EncoderAction, KeyAction};
use rmk::{encoder, k, layer};

pub(crate) const COL: usize = 15;
pub(crate) const ROW: usize = 2;
pub(crate) const NUM_LAYER: usize = 1;
pub(crate) const NUM_ENCODER: usize = 3;

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
        ];
    }

    #[cfg(not(feature = "sensor-rotated-180"))]
    {
    [
        layer!([
            [
                k!(MouseBtn2), k!(A), k!(B), k!(C),
                k!(E), k!(G), k!(MouseBtn1), k!(H),
                k!(I), k!(J), k!(K), k!(L),
                k!(Kc5), k!(Kc6), k!(Kc7)
            ],
            [
                k!(N), k!(M), k!(P), k!(O),
                k!(D), k!(F), k!(S), k!(Q),
                k!(R), k!(T), k!(V), k!(U),
                k!(Kc0), k!(Kc9), k!(Kc8)
            ]
        ]),
    ]
    }
}

pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    #[cfg(feature = "sensor-rotated-180")]
    {
        return [
            [
                encoder!(k!(Kc3), k!(Kc4)),
                encoder!(k!(Kc1), k!(Kc2)),
                encoder!(k!(MouseWheelUp), k!(MouseWheelDown)),
            ],
        ];
    }

    #[cfg(not(feature = "sensor-rotated-180"))]
    {
    [
        [
            encoder!(k!(Kc1), k!(Kc2)),
            encoder!(k!(MouseWheelUp), k!(MouseWheelDown)),
            encoder!(k!(Kc3), k!(Kc4)),
        ],
    ]
    }
}
