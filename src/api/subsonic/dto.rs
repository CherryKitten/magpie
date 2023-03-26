use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Eq, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubsonicResponse {
    pub magpie_version: String,
    pub license: Option<SubsonicLicense>,
    pub status: SubsonicStatus,
    pub r#type: String,
    pub version: String,
}

impl SubsonicResponse {
    pub(crate) fn new() -> Self {
        SubsonicResponse {
            version: "1.16.1".to_string(),
            magpie_version: "0.2.0".to_string(),
            r#type: "Magpie".to_string(),

            ..Default::default()
        }
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SubsonicStatus {
    #[default]
    Ok,
    Unimplemented,
    Error,
}

#[derive(Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct SubsonicLicense {
    valid: String,
    email: String,
    license_expires: String,
}

impl Default for SubsonicLicense {
    fn default() -> Self {
        let expiry = match chrono::Utc::now().checked_add_months(chrono::Months::new(12)) {
            None => chrono::Utc::now(),
            Some(x) => x,
        };

        SubsonicLicense {
            valid: String::from("true"),
            email: String::from("alwaysvalid@example.com"),
            license_expires: expiry.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        }
    }
}

impl SubsonicLicense {
    pub(crate) fn new() -> Self {
        SubsonicLicense::default()
    }
}
