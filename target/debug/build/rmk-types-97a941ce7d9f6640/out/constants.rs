#[allow(clippy::redundant_static_lifetimes)]
pub const MOUSE_KEY_INTERVAL: u16 = 20;
#[allow(clippy::redundant_static_lifetimes)]
pub const MOUSE_WHEEL_INTERVAL: u16 = 80;
#[allow(clippy::redundant_static_lifetimes)]
pub const COMBO_MAX_NUM: usize = 8;
#[allow(clippy::redundant_static_lifetimes)]
pub const COMBO_MAX_LENGTH: usize = 4;
#[allow(clippy::redundant_static_lifetimes)]
pub const MACRO_SPACE_SIZE: usize = 256;
#[allow(clippy::redundant_static_lifetimes)]
pub const FORK_MAX_NUM: usize = 8;
#[allow(clippy::redundant_static_lifetimes)]
pub const DEBOUNCE_THRESHOLD: u16 = 20;
#[allow(clippy::redundant_static_lifetimes)]
pub const REPORT_CHANNEL_SIZE: usize = 16;
#[allow(clippy::redundant_static_lifetimes)]
pub const VIAL_CHANNEL_SIZE: usize = 4;
#[allow(clippy::redundant_static_lifetimes)]
pub const FLASH_CHANNEL_SIZE: usize = 4;
#[allow(clippy::redundant_static_lifetimes)]
pub const SPLIT_PERIPHERALS_NUM: usize = 0;
#[allow(clippy::redundant_static_lifetimes)]
pub const NUM_BLE_PROFILE: usize = 3;
#[allow(clippy::redundant_static_lifetimes)]
pub const SPLIT_CENTRAL_SLEEP_TIMEOUT_SECONDS: u32 = 0;
#[allow(clippy::redundant_static_lifetimes)]
pub const MORSE_MAX_NUM: usize = 8;
#[allow(clippy::redundant_static_lifetimes)]
pub const MAX_PATTERNS_PER_KEY: usize = 8;
#[allow(clippy::redundant_static_lifetimes)]
pub const BLE_STATUS_CHANGE_EVENT_CHANNEL_SIZE: usize = 2;
#[allow(clippy::redundant_static_lifetimes)]
pub const BLE_STATUS_CHANGE_EVENT_PUB_SIZE: usize = 2;
#[allow(clippy::redundant_static_lifetimes)]
pub const BLE_STATUS_CHANGE_EVENT_SUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const CONNECTION_CHANGE_EVENT_CHANNEL_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const CONNECTION_CHANGE_EVENT_PUB_SIZE: usize = 2;
#[allow(clippy::redundant_static_lifetimes)]
pub const CONNECTION_CHANGE_EVENT_SUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const MODIFIER_EVENT_CHANNEL_SIZE: usize = 8;
#[allow(clippy::redundant_static_lifetimes)]
pub const MODIFIER_EVENT_PUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const MODIFIER_EVENT_SUB_SIZE: usize = 2;
#[allow(clippy::redundant_static_lifetimes)]
pub const KEYBOARD_EVENT_CHANNEL_SIZE: usize = 16;
#[allow(clippy::redundant_static_lifetimes)]
pub const KEYBOARD_EVENT_PUB_SIZE: usize = 2;
#[allow(clippy::redundant_static_lifetimes)]
pub const KEYBOARD_EVENT_SUB_SIZE: usize = 2;
#[allow(clippy::redundant_static_lifetimes)]
pub const LAYER_CHANGE_EVENT_CHANNEL_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const LAYER_CHANGE_EVENT_PUB_SIZE: usize = 2;
#[allow(clippy::redundant_static_lifetimes)]
pub const LAYER_CHANGE_EVENT_SUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const WPM_UPDATE_EVENT_CHANNEL_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const WPM_UPDATE_EVENT_PUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const WPM_UPDATE_EVENT_SUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const LED_INDICATOR_EVENT_CHANNEL_SIZE: usize = 2;
#[allow(clippy::redundant_static_lifetimes)]
pub const LED_INDICATOR_EVENT_PUB_SIZE: usize = 2;
#[allow(clippy::redundant_static_lifetimes)]
pub const LED_INDICATOR_EVENT_SUB_SIZE: usize = 3;
#[allow(clippy::redundant_static_lifetimes)]
pub const SLEEP_STATE_EVENT_CHANNEL_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const SLEEP_STATE_EVENT_PUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const SLEEP_STATE_EVENT_SUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const BATTERY_STATE_EVENT_CHANNEL_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const BATTERY_STATE_EVENT_PUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const BATTERY_STATE_EVENT_SUB_SIZE: usize = 0;
#[allow(clippy::redundant_static_lifetimes)]
pub const BATTERY_ADC_EVENT_CHANNEL_SIZE: usize = 2;
#[allow(clippy::redundant_static_lifetimes)]
pub const BATTERY_ADC_EVENT_PUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const BATTERY_ADC_EVENT_SUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const CHARGING_STATE_EVENT_CHANNEL_SIZE: usize = 2;
#[allow(clippy::redundant_static_lifetimes)]
pub const CHARGING_STATE_EVENT_PUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const CHARGING_STATE_EVENT_SUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const POINTING_EVENT_CHANNEL_SIZE: usize = 8;
#[allow(clippy::redundant_static_lifetimes)]
pub const POINTING_EVENT_PUB_SIZE: usize = 2;
#[allow(clippy::redundant_static_lifetimes)]
pub const POINTING_EVENT_SUB_SIZE: usize = 2;
#[allow(clippy::redundant_static_lifetimes)]
pub const PERIPHERAL_CONNECTED_EVENT_CHANNEL_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const PERIPHERAL_CONNECTED_EVENT_PUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const PERIPHERAL_CONNECTED_EVENT_SUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const CENTRAL_CONNECTED_EVENT_CHANNEL_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const CENTRAL_CONNECTED_EVENT_PUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const CENTRAL_CONNECTED_EVENT_SUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const PERIPHERAL_BATTERY_EVENT_CHANNEL_SIZE: usize = 2;
#[allow(clippy::redundant_static_lifetimes)]
pub const PERIPHERAL_BATTERY_EVENT_PUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const PERIPHERAL_BATTERY_EVENT_SUB_SIZE: usize = 2;
#[allow(clippy::redundant_static_lifetimes)]
pub const CLEAR_PEER_EVENT_CHANNEL_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const CLEAR_PEER_EVENT_PUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const CLEAR_PEER_EVENT_SUB_SIZE: usize = 0;
#[allow(clippy::redundant_static_lifetimes)]
pub const ACTION_EVENT_CHANNEL_SIZE: usize = 16;
#[allow(clippy::redundant_static_lifetimes)]
pub const ACTION_EVENT_PUB_SIZE: usize = 1;
#[allow(clippy::redundant_static_lifetimes)]
pub const ACTION_EVENT_SUB_SIZE: usize = 0;