use bevy::prelude::*;
use bevy_settings::{Deserialize, PersistSetting, PersistSettings, Serialize, SettingsPlugin};

#[derive(Resource, Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(crate = "bevy_settings::serde")]
pub struct CargoSettings {}

impl Default for CargoSettings {
    fn default() -> Self {
        Self {}
    }
}
