use hbb_common::config::{self, keys};

const CUSTOM_APP_NAME: Option<&str> = Some("Reuter IT Networks");
const CUSTOM_ID_SERVER: &str = "186.123.149.250";
const CUSTOM_RELAY_SERVER: &str = "186.123.149.250:21117";
const CUSTOM_API_SERVER: &str = "";
const CUSTOM_SERVER_KEY: &str = "N8Rz1kh+Hz318VzI2q4l7qUccsCm6kOXAyfL3Biv+Qg=";

pub fn apply() {
    if let Some(app_name) = CUSTOM_APP_NAME {
        *config::APP_NAME.write().unwrap() = app_name.to_owned();
    }

    let mut defaults = config::DEFAULT_SETTINGS.write().unwrap();
    defaults
        .entry(keys::OPTION_CUSTOM_RENDEZVOUS_SERVER.to_owned())
        .or_insert_with(|| CUSTOM_ID_SERVER.to_owned());
    defaults
        .entry(keys::OPTION_RELAY_SERVER.to_owned())
        .or_insert_with(|| CUSTOM_RELAY_SERVER.to_owned());
    defaults
        .entry("api-server".to_owned())
        .or_insert_with(|| CUSTOM_API_SERVER.to_owned());
    defaults
        .entry("key".to_owned())
        .or_insert_with(|| CUSTOM_SERVER_KEY.to_owned());
}
