use bevy::prelude::*;

use crate::launcher_ui::{
    styles::{GAP, LG_PADDING, TERTIARY_COLOR},
    search_bar::search_top_bar,
};

use super::content_area_plugin::ContentArea;

pub(crate) fn learn_content_area(parent: Entity) -> impl Bundle {
    (
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Stretch,
                overflow: Overflow::hidden(),
                row_gap: GAP,
                ..default()
            },
            BorderRadius::all(Val::Px(10.0)),
            ContentArea::Projects,
            ChildOf(parent),
        ),
        children![search_top_bar(), learn_grid_list()],
    )
}

fn grid_list_item() -> impl Bundle {
    (
        Node {
            width: Val::Px(200.0),
            height: Val::Px(200.0),
            flex_shrink: 0.0,
            display: Display::Flex,
            ..default()
        },
        BackgroundColor(TERTIARY_COLOR),
    )
}
/// Creates a grid list of items for the learn content area.
/// TODO: Make the grid a proper scroll box to allow for more items to be added.
/// This should be expanded to be reused in the community content area and the assets content area.
pub(crate) fn learn_grid_list() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            column_gap: GAP,
            row_gap: GAP,
            flex_wrap: FlexWrap::Wrap,
            align_content: AlignContent::Start,
            padding: LG_PADDING,
            overflow: Overflow::scroll_y(),
            ..default()
        },
        BackgroundColor(Color::hsla(0.0, 0.0, 0.7, 1.0)),
        children![
            grid_list_item(),
            grid_list_item(),
            grid_list_item(),
            grid_list_item(),
            grid_list_item(),
            grid_list_item(),
            grid_list_item(),
            grid_list_item(),
            grid_list_item(),
            grid_list_item(),
            grid_list_item(),
        ],
    )
}
