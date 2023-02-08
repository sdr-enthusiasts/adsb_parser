use crate::MessageResult;
use serde::{Deserialize, Serialize};

/// Trait for performing a decode if you wish to apply it to types other than the defaults done in this library.
///
/// The originating data must be in JSON format and have support for providing a `str`, and will not consume the source.
///
/// This is intended for specifically decoding to `ADSBMessage`.
pub trait NewADSBJsonMessage {
    fn to_adsb(&self) -> MessageResult<ADSBJsonMessage>;
}

/// Implementing `.to_adsb()` for the type `String`.
///
/// This does not consume the `String`.
impl NewADSBJsonMessage for String {
    fn to_adsb(&self) -> MessageResult<ADSBJsonMessage> {
        serde_json::from_str(self)
    }
}

/// Supporting `.to_adsb()` for the type `str`.
///
/// This does not consume the `str`.
impl NewADSBJsonMessage for str {
    fn to_adsb(&self) -> MessageResult<ADSBJsonMessage> {
        serde_json::from_str(self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct ADSBJsonMessage {
    pub now: f64,    // Unix timestamp
    pub hex: String, // ICAO address
    #[serde(rename = "type")]
    pub adsb_type: String, // ADSB type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight: Option<String>, // callsign
    #[serde(rename = "r")]
    pub aircraft_registration: String, // registration
    #[serde(skip_serializing_if = "Option::is_none", rename = "t")]
    pub aircraft_type: Option<String>,
    pub alt_baro: Altitude, // altitude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_geom: Option<i32>, // altitude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gs: Option<f32>, // ground speed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<f32>, // track
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baro_rate: Option<i32>, // vertical rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geom_rate: Option<i32>, // vertical rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squawk: Option<String>, // squawk
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency: Option<String>, // emergency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>, // category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_qnh: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_altitude_mcp: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_heading: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub true_heading: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_modes: Option<Vec<NavModes>>,
    pub lat: f32, // latitude
    pub lon: f32, // longitude
    pub nic: i32, // Navigation Integrity Category
    pub rc: i32,  // Radius of Containment, meter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen_pos: Option<f64>, // how long ago (in seconds before "now") the position was last updated
    pub seen: f64, // how long ago (in seconds before "now") the message was last received
    pub r_dst: f32, // distance from receiver
    pub r_dir: f32,
    pub version: i32, // version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nic_baro: Option<i8>, // Navigation Integrity Category for Barometric Altitude (2.2.5.1.35)
    pub nac_p: i8,    // Navigation Accuracy Category for Position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nac_v: Option<i8>, // Navigation Accuracy Category for Velocity
    pub sil: i8,      // Source Integrity Level
    pub sil_type: SilType, // Source Integrity Level for Type of Aircraft
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gva: Option<i8>, // Geometric Vertical Accuracy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sda: Option<i8>, // System Design Assurance (2.2.3.2.7.2.4.6)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<i8>, // Alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spi: Option<i8>, // Flight status special position identification bit (2.2.3.2.3.2)
    pub mlat: Vec<String>, // MLAT
    pub tisb: Vec<String>, // TIS-B
    pub messages: i32, // number of messages
    pub rssi: f32,
    #[serde(skip_serializing_if = "Option::is_none", rename = "dbFlags")]
    pub dbflags: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calc_track: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Altitude {
    I32(i32),
    #[serde(rename = "ground")]
    Ground(String),
}

impl Default for Altitude {
    fn default() -> Self {
        Altitude::I32(0)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NavModes {
    #[serde(rename = "althold")]
    AltHold,
    #[serde(rename = "autopilot")]
    AutoPilot,
    #[serde(rename = "vnav")]
    VNAV,
    #[serde(rename = "tcas")]
    TCAS,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SilType {
    #[serde(rename = "perhour")]
    PerHour,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for SilType {
    fn default() -> Self {
        SilType::Unknown
    }
}

impl ADSBJsonMessage {
    /// Converts `ADSBsMessage` to `String`.
    pub fn to_string(&self) -> MessageResult<String> {
        serde_json::to_string(self)
    }

    /// Converts `ADSBJsonMessage` to `String` and appends a `\n` to the end.
    pub fn to_string_newline(&self) -> MessageResult<String> {
        match serde_json::to_string(self) {
            Err(to_string_error) => Err(to_string_error),
            Ok(string) => Ok(format!("{}\n", string)),
        }
    }

    /// Converts `ADSBJsonMessage` to a `String` encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    pub fn to_bytes(&self) -> MessageResult<Vec<u8>> {
        match self.to_string() {
            Err(conversion_failed) => Err(conversion_failed),
            Ok(string) => Ok(string.into_bytes()),
        }
    }

    /// Converts `ADSBJsonMessage` to a `String` terminated with a `\n` and encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    pub fn to_bytes_newline(&self) -> MessageResult<Vec<u8>> {
        match self.to_string_newline() {
            Err(conversion_failed) => Err(conversion_failed),
            Ok(string) => Ok(string.into_bytes()),
        }
    }
}
