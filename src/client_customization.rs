use base::config::keys::{OPTION_APPROVE_MODE, OPTION_CUSTOM_RENDEZVOUS_SERVER, OPTION_RELAY_SERVER, OPTION_VERIFICATION_METHOD};
use hbb_common::config::{self};

const CUSTOM_APP_NAME: Option<&str> = Some("Reuter IT Networks");
const CUSTOM_ID_SERVER: &str = "186.123.149.250:21116";
const CUSTOM_RELAY_SERVER: &str = "186.123.149.250:21117";
const CUSTOM_API_SERVER: &str = "";
const CUSTOM_SERVER_KEY: &str = "N8Rz1kh+Hz318VzI2q4l7qUccsCm6kOXAyfL3Biv+Qg=";
const CUSTOM_PRESET_PASSWORD_STORAGE: &str = "00T59vhMY2XOJp0s0RUn8A2uqAm0e+o9rugVX7Dp8L3rQ=";
const CUSTOM_PRESET_PASSWORD_SALT: &str = "reuter-it-networks-preset-password-2026";

pub fn apply() {
    if let Some(app_name) = CUSTOM_APP_NAME {
        *config::APP_NAME.write().unwrap() = app_name.to_owned();
    }

    let mut defaults = config::DEFAULT_SETTINGS.write().unwrap();
    defaults
        .entry(OPTION_CUSTOM_RENDEZVOUS_SERVER.to_owned())
        .or_insert_with(|| CUSTOM_ID_SERVER.to_owned());
    defaults
        .entry(OPTION_RELAY_SERVER.to_owned())
        .or_insert_with(|| CUSTOM_RELAY_SERVER.to_owned());
    defaults
        .entry("api-server".to_owned())
        .or_insert_with(|| CUSTOM_API_SERVER.to_owned());
    defaults
        .entry("key".to_owned())
        .or_insert_with(|| CUSTOM_SERVER_KEY.to_owned());

    let mut overrides = config::OVERWRITE_SETTINGS.write().unwrap();
    overrides
        .entry(OPTION_APPROVE_MODE.to_owned())
        .or_insert_with(|| "password".to_owned());
    overrides
        .entry(OPTION_VERIFICATION_METHOD.to_owned())
        .or_insert_with(|| "use-permanent-password".to_owned());

    let mut hard_settings = config::HARD_SETTINGS.write().unwrap();
    hard_settings
        .entry("password".to_owned())
        .or_insert_with(|| CUSTOM_PRESET_PASSWORD_STORAGE.to_owned());
    hard_settings
        .entry("salt".to_owned())
        .or_insert_with(|| CUSTOM_PRESET_PASSWORD_SALT.to_owned());
}
