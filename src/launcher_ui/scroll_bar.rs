use bevy::prelude::*;

use super::styles::{GAP, LG_GAP, MD_ROUNDING, SECONDARY_COLOR, SM_PADDING, TERTIARY_COLOR};


/// Creates a scroll bar for the launcher UI.
/// The scroll bar is a vertical bar that can be used to scroll through content.
/// TODO: Make the scroll bar functional.
/// TODO: This should be expanded to be reused for any horizontal or vertical scroll bar.
/// TODO: Balance the colors to match the theme of the launcher.
/// TODO: Make the scroll bar handle draggable.
/// TODO: Make the scroll bar handle change size based on the content size.
pub(crate) fn scroll_bar() -> impl Bundle {
    (
        (
            Node {
                width: Val::Auto,
                height: Val::Percent(99.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Stretch,
                margin: UiRect::vertical(LG_GAP),
                column_gap: GAP,
                overflow: Overflow::clip(),
                padding: SM_PADDING,
                ..default()
            },
            BackgroundColor(SECONDARY_COLOR),
            MD_ROUNDING,
        ),
        children![scroll_bar_handle()],
    )
}

pub(crate) fn scroll_bar_handle() -> impl Bundle {
    ((
        Node {
            width: Val::Px(10.0),
            height: Val::Px(50.0),
            display: Display::Flex,
            ..default()
        },
        BackgroundColor(TERTIARY_COLOR),
        MD_ROUNDING,
    ),)
}
