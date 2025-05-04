use bevy::prelude::*;

use super::content_area_plugin::ContentArea;

/// This module contains the UI for the project templates content area of the launcher.
/// The templates should be displayed in a grid list format with a search bar at the top.
/// The templates should be filterable by multiple categories.
/// TODO: Design the templates UI.
/// TODO: Add a search bar to the templates UI.
/// TODO: Add a templates list to the templates UI.
/// TODO: Pull list of templates from the bevy_assets repository.
/// TODO: Add a filter to the templates UI.
/// TODO: Approach CLI Team about method to add templates to the cli.
/// TODO: Use the CLI to use the templates to create a new projects.
pub(crate) fn templates_content_area(parent: Entity) -> impl Bundle {
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
