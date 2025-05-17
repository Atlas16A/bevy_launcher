use bevy::prelude::*;

use crate::launcher_ui::styles::{
    LG_GAP, MD_ROUNDING, PRIMARY_COLOR, SECONDARY_COLOR, TERTIARY_COLOR, XL_GAP, XL_PADDING,
    lg_font_center, md_font_center,
};

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
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: XL_PADDING,
            ..default()
        },
        BackgroundColor(PRIMARY_COLOR),
        BorderRadius::all(Val::Px(10.0)),
        ContentArea::Settings,
        ChildOf(parent),
        children![settings_page_header(), settings_content()],
    )
}

fn settings_page_header() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Auto,
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: XL_PADDING,
            ..default()
        },
        children![(
            Node {
                width: Val::Percent(75.0),
                height: Val::Auto,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: XL_PADDING,
                border: UiRect::bottom(Val::Px(2.0)),
                ..default()
            },
            BorderColor(SECONDARY_COLOR),
            children![(lg_font_center("Launcher settings".to_string()),),]
        )],
    )
}

fn settings_content() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            row_gap: XL_GAP,
            ..default()
        },
        children![settings_categories(), settings_listing()],
    )
}

fn settings_categories() -> impl Bundle {
    (
        Node {
            width: Val::Auto,
            height: Val::Auto,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            padding: XL_PADDING,
            row_gap: LG_GAP,
            ..default()
        },
        children![settings_category("General"), settings_category("Assets")],
    )
}

fn settings_category(text: impl Into<String>) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Auto,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: XL_PADDING,
            ..default()
        },
        BackgroundColor(SECONDARY_COLOR),
        MD_ROUNDING,
        children![md_font_center(text)],
    )
}

fn settings_listing() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            row_gap: LG_GAP,
            ..default()
        },
        children![
            settings_category_block("Banner"),
            settings_category_block("Something else"),
            settings_category_block("Another thing"),
        ],
    )
}

fn settings_category_block(text: impl Into<String>) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Auto,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            padding: XL_PADDING,
            row_gap: LG_GAP,
            ..default()
        },
        BackgroundColor(SECONDARY_COLOR),
        MD_ROUNDING,
        children![
            (
                Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Start,
                    padding: XL_PADDING,
                    row_gap: LG_GAP,
                    ..default()
                },
                children![lg_font_center(text)]
            ),
            (
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: LG_GAP,
                    ..default()
                },
                children![
                    lg_font_center("Settings 1".to_string()),
                    lg_font_center("Settings 2".to_string()),
                    lg_font_center("Settings 3".to_string()),
                ]
            ),
        ],
    )
}
