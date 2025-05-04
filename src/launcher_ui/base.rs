use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::HoverMap,
    prelude::*,
};

use crate::{LauncherUISet, cargo_env_plugin::CargoEnv};

use super::{
    content_area::content_area_plugin::{ContentArea, ContentAreaPlugin},
    sidebar::{SideBarButton, SocialLinks, side_bar, socials_observer},
    styles::{GAP, LG_GAP, LG_PADDING, SM_PADDING, TERTIARY_COLOR, md_font_center},
};

/// Ui is laid out in a specific fashion based off the figma design
/// The layout is as follows:
/// |-------------------------------------|
/// |                Root                 |
/// | |---------------------------------| |
/// | |             Banner              | |
/// | |---------------------------------| |
/// | |---------------------------------| |
/// | |            Main Area            | |
/// | | |---------|  |----------------| | |
/// | | |         |  |                | | |
/// | | | SideBar |  |   ContentArea  | | |
/// | | |         |  |                | | |
/// | | |---------|  |----------------| | |
/// | |---------------------------------| |
/// | |             Footer              | |
/// | |---------------------------------| |
/// |-------------------------------------|
/// The root node is the parent of all other nodes
/// Named items in this layout are components you can query for.
/// The banner is a optional image area that can be enabled or disabled in the settings.
pub struct LauncherUIPlugin;

impl Plugin for LauncherUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreStartup,
            (spawn_base_ui, add_observers).chain().in_set(LauncherUISet),
        );
        app.add_systems(Update, update_scroll_position);
        app.add_plugins(ContentAreaPlugin);
    }
}

pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (dx, dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (mouse_wheel_event.x * 16.0, mouse_wheel_event.y * 16.0),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}

#[derive(Component)]
pub struct Root;

pub(crate) fn spawn_base_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cargo_env: Res<CargoEnv>,
) {
    commands.spawn(root(asset_server, &cargo_env));
}

/// Adds observers to entities that are persistent across the launcher UI.
/// Currently this includes the sidebar buttons and the social links.
fn add_observers(
    mut commands: Commands,
    sidebar_buttons: Query<Entity, With<SideBarButton>>,
    social_links: Query<Entity, With<SocialLinks>>,
) {
    for button in sidebar_buttons.iter() {
        commands
            .entity(button)
            .observe(hover_over_effect)
            .observe(hover_out_effect)
            .observe(sidebar_state_change);
    }
    for button in social_links.iter() {
        commands
            .entity(button)
            .observe(socials_observer)
            .observe(hover_over_effect)
            .observe(hover_out_effect);
    }
}

fn hover_over_effect(
    trigger: Trigger<Pointer<Over>>,
    mut background_query: Query<&mut BackgroundColor>,
    donate_query: Query<&SideBarButton>,
) {
    let mut background = background_query.get_mut(trigger.target()).unwrap();

    let donate_button = donate_query.get(trigger.target());
    if donate_button.is_ok() {
        if let SideBarButton::Donate = donate_button.unwrap() {
            background.0 = Color::hsla(328.0, 0.33, 0.27, 1.0);
        } else {
            background.0 = Color::hsla(0.0, 0.0, 0.24, 1.0);
        }
    }
}

fn hover_out_effect(
    trigger: Trigger<Pointer<Out>>,
    mut background_query: Query<&mut BackgroundColor>,
) {
    let mut background = background_query.get_mut(trigger.target()).unwrap();
    background.0 = Color::hsla(0.0, 0.0, 0.16, 1.0);
}

fn sidebar_state_change(
    trigger: Trigger<Pointer<Click>>,
    sidebar_buttons: Query<&SideBarButton>,
    mut next_state: ResMut<NextState<ContentArea>>,
) {
    match sidebar_buttons.get(trigger.target()).unwrap() {
        SideBarButton::Project => {
            next_state.set(ContentArea::Projects);
        }
        SideBarButton::Templates => {
            next_state.set(ContentArea::Templates);
        }
        SideBarButton::Installs => {
            next_state.set(ContentArea::Installs);
        }
        SideBarButton::Assets => {
            next_state.set(ContentArea::Assets);
        }
        SideBarButton::Learn => {
            next_state.set(ContentArea::Learn);
        }
        SideBarButton::Community => {
            next_state.set(ContentArea::Community);
        }
        SideBarButton::Donate => {
            next_state.set(ContentArea::Donate);
        }
        SideBarButton::Settings => {
            next_state.set(ContentArea::Settings);
        }
    }
}

/// Creates the footer of the launcher UI
/// The footer contains the version number of the launcher
/// TODO: Add error display system to show errors in the footer
fn footer(cargo_env: &CargoEnv) -> impl Bundle {
    (
        (
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                width: Val::Auto,
                height: Val::Px(26.0),
                padding: LG_PADDING,
                row_gap: LG_GAP,
                ..default()
            },
            BackgroundColor(TERTIARY_COLOR),
        ),
        children![(md_font_center(cargo_env.cargo_pkg_version.clone()),)],
    )
}

#[derive(Component)]
/// Identifier component for the main area of the launcher,
pub(crate) struct MainArea;

fn main_area(images: LauncherIconImages) -> impl Bundle {
    (
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Stretch,
                column_gap: GAP,
                overflow: Overflow::hidden(),
                ..default()
            },
            BackgroundColor(TERTIARY_COLOR),
            MainArea,
        ),
        children![side_bar(images)],
    )
}

fn root(asset_server: Res<AssetServer>, cargo_env: &CargoEnv) -> impl Bundle {
    let images = icon_asset_load(asset_server.clone());

    (
        (
            Camera2d,
            Camera {
                clear_color: ClearColorConfig::Custom(Color::hsla(0.0, 0.0, 0.09, 1.0)),
                ..default()
            },
        ),
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: SM_PADDING,
                row_gap: GAP,
                ..default()
            },
            BackgroundColor(TERTIARY_COLOR),
            Root,
        ),
        children![main_area(images), footer(cargo_env)],
    )
}

/// To avoid passing around the asset server everywhere, we load all the icon assets here
/// and return a struct containing all the handles.
/// This is a temporary solution until we have a better way to handle assets, aka Construct.
fn icon_asset_load(asset_server: AssetServer) -> LauncherIconImages {
    LauncherIconImages {
        projects_icon: asset_server.load("icons/box.png"),
        templates_icon: asset_server.load("icons/box-1.png"),
        installs_icon: asset_server.load("icons/package-plus.png"),
        assets_icon: asset_server.load("icons/boxes.png"),
        learn_icon: asset_server.load("icons/graduation-cap.png"),
        community_icon: asset_server.load("icons/users.png"),
        donate_icon: asset_server.load("icons/heart.png"),
        settings_icon: asset_server.load("icons/settings.png"),
        mastadon_icon: asset_server.load("icons/mastodon.png"),
        discord_icon: asset_server.load("icons/discord.png"),
        reddit_icon: asset_server.load("icons/reddit.png"),
        website_icon: asset_server.load("icons/website.png"),
        github_icon: asset_server.load("icons/github.png"),
        bluesky_icon: asset_server.load("icons/bluesky.png"),
    }
}

pub struct LauncherIconImages {
    pub projects_icon: Handle<Image>,
    pub templates_icon: Handle<Image>,
    pub installs_icon: Handle<Image>,
    pub assets_icon: Handle<Image>,
    pub learn_icon: Handle<Image>,
    pub community_icon: Handle<Image>,
    pub donate_icon: Handle<Image>,
    pub settings_icon: Handle<Image>,
    pub mastadon_icon: Handle<Image>,
    pub discord_icon: Handle<Image>,
    pub reddit_icon: Handle<Image>,
    pub github_icon: Handle<Image>,
    pub website_icon: Handle<Image>,
    pub bluesky_icon: Handle<Image>,
}
