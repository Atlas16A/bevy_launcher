use bevy::prelude::*;

use crate::launcher_ui::styles::{
    LG_PADDING, PRIMARY_COLOR, SECONDARY_COLOR, XL_GAP, custom_size_font_center,
    custom_size_font_right, lg_font_center, lg_font_right, md_font_left,
};

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
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(PRIMARY_COLOR),
        BorderRadius::all(Val::Px(10.0)),
        ContentArea::Donate,
        ChildOf(parent),
        children![support_bevy_text()],
    ),)
}

fn support_bevy_text() -> impl Bundle {
    (
        Node {
            width: Val::Auto,
            height: Val::Auto,
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(100.0),
            ..default()
        },
        children![
        (
            Node {
                width: Val::Auto,
                height: Val::Auto,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: LG_PADDING,
                row_gap: Val::Px(22.0),
                ..default()
            },
            children![custom_size_font_center("Support Bevy", 40.0), md_font_left("Donate to Bevy Foundation and support our mission\nto develop and support the free and open source\nBevy Engine.".to_string())]
        ), 
        (
            Node {
                width: Val::Auto,
                height: Val::Auto,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::End,
                padding: LG_PADDING,
                row_gap: Val::Px(5.0),
                ..default()
            }, 
            children![
                (
                    Node {
                        width: Val::Auto,
                        height: Val::Auto,
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::Center,
                        padding: LG_PADDING,
                        column_gap: XL_GAP,
                        border: UiRect::bottom(Val::Px(3.0)),
                        ..default()
                    }, 
                    BorderColor(SECONDARY_COLOR), 
                    children![custom_size_font_right("$14,760", 24.0), lg_font_right("Per Month".to_string())]
                ), 
                (
                    Node {
                        width: Val::Auto,
                        height: Val::Auto,
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::Center,
                        padding: LG_PADDING,
                        column_gap: XL_GAP,
                        border: UiRect::bottom(Val::Px(3.0)),
                        ..default()
                    }, 
                    BorderColor(SECONDARY_COLOR), 
                    children![custom_size_font_right("174", 24.0), lg_font_right("Members".to_string())]
                ), 
                (   
                    Node {
                        width: Val::Auto,
                        height: Val::Auto,
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::Center,
                        padding: LG_PADDING,
                        column_gap: XL_GAP,
                        border: UiRect::bottom(Val::Px(3.0)),
                        ..default()
                    }, 
                    BorderColor(SECONDARY_COLOR), 
                    children![custom_size_font_right("3", 24.0), lg_font_right("Sponsors".to_string())]
                )
            ]
        )],
    )
}
