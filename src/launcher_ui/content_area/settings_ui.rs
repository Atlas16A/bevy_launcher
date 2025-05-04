use bevy::prelude::*;

use super::content_area_plugin::ContentArea;

/// This module contains the UI for the settings content area of the launcher.
/// The settings should be displayed in a list format with a search bar at the top.
/// The settings should be divided up by category, a general rule of thumb is if its in a new struct it probably should be in a new category-
/// -Or should be sub settings of a setting.
/// Settings should be first added in the launcher_settings.rs file.
/// TODO: Design the settings UI.
/// TODO: Add a search bar to the settings UI.
/// TODO: Add a settings list to the settings UI.
/// TODO: Somehow analyze the settings struct to automate the settings UI generation.
pub(crate) fn settings_content_area(parent: Entity) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            ..default()
        },
        BackgroundColor(Color::hsla(0.0, 0.0, 0.7, 1.0)),
        BorderRadius::all(Val::Px(10.0)),
        ContentArea::Projects,
        ChildOf(parent),
    )
}
