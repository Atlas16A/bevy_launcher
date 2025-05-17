use bevy::prelude::*;
use bevy_settings::{Deserialize, PersistSetting, PersistSettings, Serialize, SettingsPlugin};

#[derive(Resource, Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(crate = "bevy_settings::serde")]
pub struct AppearanceSettings {
    pub dark_mode: bool,
    pub banner: BannerSetting,
    pub language: LanguageSetting,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            dark_mode: true,
            banner: BannerSetting::default(),
            language: LanguageSetting::English,
        }
    }
}

#[derive(Resource, Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(crate = "bevy_settings::serde")]
pub struct BannerSetting {
    pub enabled: bool,
}

impl Default for BannerSetting {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Resource, Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(crate = "bevy_settings::serde")]
pub enum LanguageSetting {
    English,
    Spanish,
    French,
    German,
    Italian,
    Portuguese,
    Russian,
    Chinese,
    Japanese,
    Korean,
}
