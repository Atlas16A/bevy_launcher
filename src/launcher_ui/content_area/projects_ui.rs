use bevy::prelude::*;

use crate::launcher_ui::{
    scroll_bar::scroll_bar,
    styles::{
        GAP, HIGHLIGHT_COLOR, LG_PADDING, MD_PADDING, MD_ROUNDING, MD_ROUNDING_VAL, PRIMARY_COLOR,
        SECONDARY_COLOR, XL_GAP, XL_PADDING, icon_placeholder, md_font_center, sm_font_center,
    },
    search_bar::search_bar,
};

use super::content_area_plugin::ContentArea;

#[derive(Component)]
struct ProjectsList;

/// The Ui for the projects list content area.
/// This is the default content area for the launcher.
/// It contains a list of projects, each with a name, path, bevy version, and last modified date.
/// The projects list is scrollable and has a top bar with a search bar.
/// The projects list is displayed in a column format.
/// TODO: Fix scrollbox sizing and overflow issues so it stops breaking the rest of the UI.
/// TODO: Handle generating the list of projects.
/// TODO: Pull projects from the settings file, (best place to put them for now).
pub(crate) fn projects_content_area(parent: Entity) -> impl Bundle {
    (
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Stretch,
                row_gap: GAP,
                //overflow: Overflow::hidden(),
                ..default()
            },
            ContentArea::Projects,
            ChildOf(parent),
        ),
        children![
            top_bar(),
            (
                (
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Center,
                        overflow: Overflow::hidden(),

                        ..default()
                    },
                    BackgroundColor(PRIMARY_COLOR),
                    MD_ROUNDING,
                    ProjectsList
                ),
                children![
                    projects_list_topbar(),
                    (
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Center,
                            overflow: Overflow::hidden(),

                            ..default()
                        },
                        children![project_list(), scroll_bar()]
                    )
                ],
            )
        ],
    )
}

fn projects_list_topbar() -> impl Bundle {
    (
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                padding: MD_PADDING,
                ..default()
            },
            BackgroundColor(SECONDARY_COLOR),
            BorderRadius::top(MD_ROUNDING_VAL),
        ),
        children![
            (
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    column_gap: GAP,
                    padding: MD_PADDING,
                    ..default()
                },
                children![
                    icon_placeholder(),
                    (
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Auto,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Start,
                            row_gap: GAP,
                            padding: MD_PADDING,
                            ..default()
                        },
                        children![md_font_center("Project".to_string())]
                    ),
                ]
            ),
            (
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    md_font_center("Bevy Version".to_string()),
                    md_font_center("Last Modified".to_string()),
                ]
            )
        ],
    )
}

pub(crate) fn project_item(
    project_name: String,
    project_path: String,
    bevy_vers: String,
    last_modified: String,
) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Auto,
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            padding: MD_PADDING,
            border: UiRect::top(Val::Px(1.0)),
            flex_shrink: 0.0,
            ..default()
        },
        BorderColor(SECONDARY_COLOR),
        Pickable {
            should_block_lower: false,
            ..default()
        },
        children![
            (
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    column_gap: XL_GAP,
                    padding: MD_PADDING,
                    ..default()
                },
                Pickable {
                    should_block_lower: false,
                    ..default()
                },
                children![
                    icon_placeholder(),
                    (
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Auto,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Start,
                            row_gap: XL_GAP,
                            padding: MD_PADDING,
                            ..default()
                        },
                        Pickable {
                            should_block_lower: false,
                            ..default()
                        },
                        children![
                            (
                                md_font_center(project_name),
                                Pickable {
                                    should_block_lower: false,
                                    ..default()
                                },
                            ),
                            (
                                sm_font_center(project_path),
                                Pickable {
                                    should_block_lower: false,
                                    ..default()
                                },
                            )
                        ]
                    ),
                ]
            ),
            (
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Pickable {
                    should_block_lower: false,
                    ..default()
                },
                children![
                    (
                        md_font_center(bevy_vers),
                        Pickable {
                            should_block_lower: false,
                            ..default()
                        },
                    ),
                    (
                        md_font_center(last_modified),
                        Pickable {
                            should_block_lower: false,
                            ..default()
                        },
                    )
                ]
            )
        ],
    )
}

fn project_list() -> impl Bundle {
    (
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                overflow: Overflow::scroll_y(),
                ..default()
            },
            BackgroundColor(PRIMARY_COLOR),
            MD_ROUNDING,
            ProjectsList,
        ),
        children![
            project_item(
                "Test Name".to_string(),
                "path/to/project...".to_string(),
                "0.16.0".to_string(),
                "6 days ago".to_string()
            ),
            project_item(
                "Test Name".to_string(),
                "path/to/project...".to_string(),
                "0.16.0".to_string(),
                "6 days ago".to_string()
            ),
            project_item(
                "Test Name".to_string(),
                "path/to/project...".to_string(),
                "0.16.0".to_string(),
                "6 days ago".to_string()
            ),
            project_item(
                "Test Name".to_string(),
                "path/to/project...".to_string(),
                "0.16.0".to_string(),
                "6 days ago".to_string()
            ),
            project_item(
                "Test Name".to_string(),
                "path/to/project...".to_string(),
                "0.16.0".to_string(),
                "6 days ago".to_string()
            ),
            project_item(
                "Test Name".to_string(),
                "path/to/project...".to_string(),
                "0.16.0".to_string(),
                "6 days ago".to_string()
            ),
            project_item(
                "Test Name".to_string(),
                "path/to/project...".to_string(),
                "0.16.0".to_string(),
                "6 days ago".to_string()
            ),
            project_item(
                "Test Name".to_string(),
                "path/to/project...".to_string(),
                "0.16.0".to_string(),
                "6 days ago".to_string()
            ),
        ],
    )
}

pub(crate) fn top_bar() -> impl Bundle {
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
        children![
            search_bar(),
            (
                (
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(36.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::Center,
                        column_gap: GAP,
                        ..default()
                    },
                    BackgroundColor(PRIMARY_COLOR),
                ),
                children![browse_button(), new_project_button()],
            )
        ],
    )
}

pub(crate) fn browse_button() -> impl Bundle {
    (
        (
            Node {
                width: Val::Auto,
                height: Val::Px(36.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: XL_GAP,
                padding: XL_PADDING,
                ..default()
            },
            BackgroundColor(SECONDARY_COLOR),
            MD_ROUNDING,
        ),
        children![md_font_center("Browse".to_string()), icon_placeholder()],
    )
}

pub(crate) fn new_project_button() -> impl Bundle {
    (
        (
            Node {
                width: Val::Auto,
                height: Val::Px(36.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: XL_GAP,
                padding: XL_PADDING,
                ..default()
            },
            BackgroundColor(HIGHLIGHT_COLOR),
            MD_ROUNDING,
        ),
        children![
            md_font_center("New Project".to_string()),
            icon_placeholder()
        ],
    )
}
