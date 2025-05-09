use bevy::{asset::load_internal_binary_asset, prelude::*, window::WindowResolution};

mod system_check_plugin;
use launcher_settings::LauncherSettingsPlugin;
use system_check_plugin::SystemCheckPlugin;

mod window_icon_plugin;
use window_icon_plugin::WindowIconPlugin;

mod launcher_ui;
use launcher_ui::{banner::BannerPlugin, base::LauncherUIPlugin};

mod cargo_env_plugin;
use cargo_env_plugin::CargoEnvPlugin;

mod launcher_settings;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Launcher".to_string(),
                resolution: WindowResolution::new(1008.0, 768.0),
                resize_constraints: WindowResizeConstraints {
                    min_width: 1008.0,
                    min_height: 700.0,
                    max_width: 1920.0,
                    max_height: 1080.0,
                },
                ..default()
            }),
            ..default()
        }),
        CargoEnvPlugin,
        LauncherUIPlugin,
        WindowIconPlugin,
        SystemCheckPlugin,
        BannerPlugin,
        LauncherSettingsPlugin,
    ));

    load_internal_binary_asset!(
        app,
        TextFont::default().font,
        "../assets/fonts/Fira_Sans/FiraSans-Regular.ttf",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );

    app.run();
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LauncherUISet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LauncherBackgroundTasksSet;
