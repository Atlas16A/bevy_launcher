use bevy::prelude::*;

/// This plugin is responsible for retrieving the Cargo metadata during compilation and creating resources that contain the metadata.
/// Currently it is used to display the version of the application in the UI.
pub struct CargoEnvPlugin;

impl Plugin for CargoEnvPlugin {
    fn build(&self, app: &mut App) {
        let cargo_pkg_version = env!("CARGO_PKG_VERSION");
        let cargo_pkg_version = cargo_pkg_version.to_string();
        app.insert_resource(CargoEnv { cargo_pkg_version });
    }
}

#[derive(Resource)]
pub struct CargoEnv {
    pub cargo_pkg_version: String,
}
