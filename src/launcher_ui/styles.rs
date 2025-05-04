use bevy::{
    color::Color,
    ecs::{bundle::Bundle, children},
    prelude::*,
    text::{JustifyText, LineHeight, TextFont, TextLayout},
    ui::{
        BackgroundColor, BorderRadius, Display, Node, UiRect, Val,
        widget::{ImageNode, Text},
    },
    utils::default,
};

/// This is a collection of styles for the Bevy Launcher UI.
/// Not all of these styles are used in the current implementation.
/// This is a work in progress and will be updated as the UI is developed.
/// The goal is to create a consistent and visually appealing UI that matches the Bevy brand.
pub(crate) const PRIMARY_COLOR: Color = Color::hsla(0.0, 0.0, 0.16, 1.0);
pub(crate) const SECONDARY_COLOR: Color = Color::hsla(0.0, 0.0, 0.24, 1.0);
pub(crate) const TERTIARY_COLOR: Color = Color::hsla(0.0, 0.0, 0.09, 1.0);
pub(crate) const TERTIARY_COLOR_OPACITY_M: Color = Color::hsla(0.0, 0.0, 0.85, 0.2);
pub(crate) const HIGHLIGHT_COLOR: Color = Color::hsla(212.0, 0.72, 0.45, 1.0);

pub(crate) const SM_ROUNDING_VAL: Val = Val::Px(4.0);
pub(crate) const MD_ROUNDING_VAL: Val = Val::Px(6.0);
pub(crate) const LG_ROUNDING_VAL: Val = Val::Px(10.0);
pub(crate) const SM_ROUNDING: BorderRadius = BorderRadius::all(SM_ROUNDING_VAL);
pub(crate) const MD_ROUNDING: BorderRadius = BorderRadius::all(MD_ROUNDING_VAL);
pub(crate) const LG_ROUNDING: BorderRadius = BorderRadius::all(LG_ROUNDING_VAL);

pub(crate) const XSM_FONT_SIZE: f32 = 11.0;
pub(crate) const SM_FONT_SIZE: f32 = 12.0;
pub(crate) const MD_FONT_SIZE: f32 = 13.0;
pub(crate) const LG_FONT_SIZE: f32 = 14.0;
pub(crate) const XL_FONT_SIZE: f32 = 16.0;

pub(crate) fn xsm_font_center(text: String) -> impl Bundle {
    (
        Text(text),
        TextFont {
            font_size: XSM_FONT_SIZE,
            line_height: LineHeight::Px(16.0),
            ..default()
        },
        TextLayout {
            justify: JustifyText::Center,
            ..default()
        },
    )
}
pub(crate) fn sm_font_center(text: String) -> impl Bundle {
    (
        Text(text),
        TextFont {
            font_size: SM_FONT_SIZE,
            line_height: LineHeight::Px(16.0),
            ..default()
        },
        TextLayout {
            justify: JustifyText::Center,
            ..default()
        },
    )
}
pub(crate) fn md_font_center(text: String) -> impl Bundle {
    (
        Text(text),
        TextFont {
            font_size: MD_FONT_SIZE,
            line_height: LineHeight::Px(16.0),
            ..default()
        },
        TextLayout {
            justify: JustifyText::Center,
            ..default()
        },
    )
}
pub(crate) fn lg_font_center(text: String) -> impl Bundle {
    (
        Text(text),
        TextFont {
            font_size: LG_FONT_SIZE,
            line_height: LineHeight::Px(16.0),
            ..default()
        },
        TextLayout {
            justify: JustifyText::Center,
            ..default()
        },
    )
}
pub(crate) fn sm_font_left(text: String) -> impl Bundle {
    (
        Text(text),
        TextFont {
            font_size: SM_FONT_SIZE,
            line_height: LineHeight::Px(16.0),
            ..default()
        },
        TextLayout {
            justify: JustifyText::Left,
            ..default()
        },
    )
}
pub(crate) fn md_font_left(text: String) -> impl Bundle {
    (
        Text(text),
        TextFont {
            font_size: MD_FONT_SIZE,
            line_height: LineHeight::Px(16.0),
            ..default()
        },
        TextLayout {
            justify: JustifyText::Left,
            ..default()
        },
    )
}
pub(crate) fn lg_font_left(text: String) -> impl Bundle {
    (
        Text(text),
        TextFont {
            font_size: LG_FONT_SIZE,
            line_height: LineHeight::Px(16.0),
            ..default()
        },
        TextLayout {
            justify: JustifyText::Left,
            ..default()
        },
    )
}
pub(crate) fn sm_font_right(text: String) -> impl Bundle {
    (
        Text(text),
        TextFont {
            font_size: SM_FONT_SIZE,
            line_height: LineHeight::Px(16.0),
            ..default()
        },
        TextLayout {
            justify: JustifyText::Right,
            ..default()
        },
    )
}
pub(crate) fn md_font_right(text: String) -> impl Bundle {
    (
        Text(text),
        TextFont {
            font_size: MD_FONT_SIZE,
            line_height: LineHeight::Px(16.0),
            ..default()
        },
        TextLayout {
            justify: JustifyText::Right,
            ..default()
        },
    )
}
pub(crate) fn lg_font_right(text: String) -> impl Bundle {
    (
        Text(text),
        TextFont {
            font_size: LG_FONT_SIZE,
            line_height: LineHeight::Px(16.0),
            ..default()
        },
        TextLayout {
            justify: JustifyText::Right,
            ..default()
        },
    )
}
pub(crate) fn xl_font_center(text: String, color: Color) -> impl Bundle {
    (
        Text(text),
        TextFont {
            font_size: XL_FONT_SIZE,
            line_height: LineHeight::Px(16.0),
            ..default()
        },
        TextLayout {
            justify: JustifyText::Center,
            ..default()
        },
        TextColor(color),
    )
}

pub(crate) const SM_PADDING: UiRect = UiRect::all(Val::Px(4.0));
pub(crate) const MD_PADDING: UiRect = UiRect::all(Val::Px(6.0));
pub(crate) const LG_PADDING: UiRect = UiRect::all(Val::Px(8.0));
pub(crate) const XL_PADDING: UiRect = UiRect::all(Val::Px(10.0));

pub(crate) const SM_GAP: Val = Val::Px(2.0);
pub(crate) const GAP: Val = Val::Px(4.0);
pub(crate) const LG_GAP: Val = Val::Px(6.0);
pub(crate) const XL_GAP: Val = Val::Px(8.0);

pub(crate) fn icon(image_handle: Handle<Image>, size: (Val, Val)) -> impl Bundle {
    (
        Node {
            width: size.0,
            height: size.1,
            display: Display::Flex,
            ..default()
        },
        children![(
            ImageNode {
                image: image_handle,
                ..default()
            },
            Node {
                width: size.0,
                height: size.1,
                ..default()
            }
        )],
    )
}

pub(crate) fn icon_placeholder() -> impl Bundle {
    (
        Node {
            width: Val::Px(16.0),
            height: Val::Px(16.0),
            display: Display::Flex,
            ..default()
        },
        BackgroundColor(Color::hsla(200.0, 0.0, 0.52, 1.0)),
    )
}
