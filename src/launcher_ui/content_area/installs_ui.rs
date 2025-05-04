use bevy::prelude::*;

use super::content_area_plugin::ContentArea;

/// This needs to be designed in the figma still.
/// This Ui should display rust install information, including:
/// - The current version of rust
/// - The current version of rustup
/// - The current version of cargo
/// - The current version of rust-analyzer
/// - The current version of rustfmt
/// - The current version of clippy
/// - The current version of rustdoc
/// - The current version of bevy-cli
/// - The compilation targets that are installed
/// - And any additional information that may be useful
///
/// This UI should also allow the user to do the following:
/// install/uninstall/update anything listed above.
/// This UI should also allow the user to view the installation logs and the installation progress.
pub(crate) fn installs_content_area(parent: Entity) -> impl Bundle {
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
