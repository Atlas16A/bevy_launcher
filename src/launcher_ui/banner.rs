use bevy::prelude::*;

use crate::launcher_settings::Settings;

use super::{base::Root, styles::MD_ROUNDING};

pub struct BannerPlugin;

#[derive(Component)]
pub(crate) struct BannerComponent;

impl Plugin for BannerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_banner);
    }
}

/// A banner for the launcher UI.
/// This banner is displayed at the top of the launcher UI and contains a background pattern and an image.
/// The background pattern is made of the Bevy logo, and the image is changed each launcher update.
pub(crate) fn banner(asset_server: Res<AssetServer>) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(140.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        // Background pattern made of the Bevy logo
        ImageNode {
            image: asset_server.load("pattern/bevy_background_pattern.png"),
            image_mode: NodeImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0,
            },
            ..default()
        },
        MD_ROUNDING,
        BannerComponent,
        children![(
            Node {
                width: Val::Px(1000.0),
                height: Val::Px(140.0),
                overflow: Overflow::clip(),
                ..default()
            },
            MD_ROUNDING,
            // Banner image
            ImageNode {
                image: asset_server.load("banners/Forest_Scene_by_IceSentry.png"),
                rect: Some(Rect {
                    min: Vec2::ZERO,
                    max: Vec2::new(1000.0, 140.0),
                }),
                ..default()
            }
        )],
    )
}

fn create_banner(
    root: Query<Entity, With<Root>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Settings>,
) {
    if !settings.banner.enabled {
        return;
    }
    let root = root.single().unwrap();
    let banner = commands.spawn(banner(asset_server)).id();
    commands.entity(root).insert_children(0, &[banner]);
}
