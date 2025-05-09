use bevy::prelude::*;

pub struct WindowIconPlugin;

impl Plugin for WindowIconPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_window_icon);
    }
}
/// Sets the window icon for the application.
/// TODO: Implement this for all platforms.
/// TODO: Use the Bevy icon.
fn set_window_icon() {}
