use bevy::prelude::*;
use bevy_settings::{Deserialize, PersistSetting, PersistSettings, Serialize, SettingsPlugin};

mod appearance;
pub use appearance::AppearanceSettings;

mod cargo;
pub use cargo::CargoSettings;

pub struct LauncherSettingsPlugin;

impl Plugin for LauncherSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_settings::SettingsPlugin::<Settings>::new(
            "bevy",
            "bevy_launcher",
        ))
        .add_systems(Startup, persist_settings);
    }
}

pub fn persist_settings(mut writer: EventWriter<PersistSettings>) {
    writer.write(PersistSettings);
}

#[derive(Resource, Default, Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(crate = "bevy_settings::serde")]
/// This struct is used to store the settings for the launcher.
/// It is used to persist the settings across sessions.
/// The settings plugin will automatically save and load the settings from the file.
/// The settings are stored in the `bevy_launcher` folder in the appropriate location for the platform, this is determined by the `bevy_settings` crate which used the Directories crate.
/// Due to the way the bevy_settings crate works, the settings are inside a main Settings struct. TODO: Change this so individual resources can be used for each setting.
/// TODO: Explore improving the bevy_settings crate's api/documentation.
pub struct Settings {
    pub appearance: AppearanceSettings,
    pub cargo: CargoSettings,
}
