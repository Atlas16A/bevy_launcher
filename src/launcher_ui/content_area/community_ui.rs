use bevy::prelude::*;

use super::content_area_plugin::ContentArea;

/// Similar to the website's community page, this content area will display a list of community
/// that are available for use in the Bevy engine.
/// This needs to be designed in the figma still.
pub(crate) fn community_content_area(parent: Entity) -> impl Bundle {
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
