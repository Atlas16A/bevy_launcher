use bevy::prelude::*;

use super::content_area_plugin::ContentArea;

/// Creates a content area for the donate UI.
/// This is a placeholder for the actual donate UI.
/// TODO: Implement the donate UI.
/// This UI should effectively replicate the current Bevy website donate page.
/// Look into embedding the payment processing if possible.
pub(crate) fn donate_content_area(parent: Entity) -> impl Bundle {
    ((
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
        ContentArea::Donate,
        ChildOf(parent),
    ),)
}
