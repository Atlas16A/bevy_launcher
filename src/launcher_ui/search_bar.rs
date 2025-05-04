use bevy::prelude::*;

use super::styles::{
    GAP, HIGHLIGHT_COLOR, LG_PADDING, MD_ROUNDING, PRIMARY_COLOR, SECONDARY_COLOR, XL_GAP,
    XL_PADDING, icon_placeholder, md_font_center,
};

pub(crate) fn search_bar() -> impl Bundle {
    (
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(36.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                padding: LG_PADDING,
                column_gap: XL_GAP,
                ..default()
            },
            BackgroundColor(SECONDARY_COLOR),
            Outline::new(Val::Px(1.0), Val::Px(0.0), Color::hsla(0.0, 0.0, 0.25, 1.0)),
            MD_ROUNDING,
        ),
        children![md_font_center("Search".to_string())],
    )
}

/// TODO: Make this actually take in a string of text
/// and display it in the search bar
/// This is a placeholder for now
/// to show the search bar in the top bar
/// and to show the search bar in the learn grid list
/// This will be replaced with a real search bar
/// when the search bar is implemented.
pub(crate) fn search_top_bar() -> impl Bundle {
    (
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(52.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                padding: LG_PADDING,
                column_gap: GAP,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(PRIMARY_COLOR),
            MD_ROUNDING,
        ),
        children![search_bar()],
    )
}
