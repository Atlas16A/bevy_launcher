use bevy::prelude::*;

use crate::launcher_ui::base::{MainArea, spawn_base_ui};

use super::{
    assets_ui::assets_content_area, community_ui::community_content_area,
    donate_ui::donate_content_area, installs_ui::installs_content_area,
    learn_ui::learn_content_area, projects_ui::projects_content_area,
    settings_ui::settings_content_area, templates_ui::templates_content_area,
};

pub struct ContentAreaPlugin;

impl Plugin for ContentAreaPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ContentArea>();
        app.add_systems(Startup, spawn_content_area.after(spawn_base_ui));
        for state in ContentArea::iter() {
            app.add_systems(OnEnter(state), spawn_content_area)
                .add_systems(OnExit(state), cleanup_content_area);
        }
    }
}

#[derive(Component, States, Debug, Clone, Eq, PartialEq, Hash, Default, Copy)]
pub enum ContentArea {
    #[default]
    Projects,
    Templates,
    Installs,
    Assets,
    Learn,
    Community,
    Donate,
    Settings,
}
impl ContentArea {
    fn iter() -> impl Iterator<Item = ContentArea> {
        [
            ContentArea::Projects,
            ContentArea::Templates,
            ContentArea::Installs,
            ContentArea::Assets,
            ContentArea::Learn,
            ContentArea::Community,
            ContentArea::Donate,
            ContentArea::Settings,
        ]
        .into_iter()
    }
}

/// This function is used to spawn the content area
/// It is called when the state changes to a new content area
fn spawn_content_area(
    mut commands: Commands,
    parent_query: Query<Entity, With<MainArea>>,
    state: Res<State<ContentArea>>,
) {
    if let Ok(parent_query) = parent_query.single() {
        match state.get() {
            ContentArea::Projects => {
                commands.spawn(projects_content_area(parent_query));
            }
            ContentArea::Templates => {
                commands.spawn(templates_content_area(parent_query));
            }
            ContentArea::Installs => {
                commands.spawn(installs_content_area(parent_query));
            }
            ContentArea::Assets => {
                commands.spawn(assets_content_area(parent_query));
            }
            ContentArea::Learn => {
                commands.spawn(learn_content_area(parent_query));
            }
            ContentArea::Community => {
                commands.spawn(community_content_area(parent_query));
            }
            ContentArea::Donate => {
                commands.spawn(donate_content_area(parent_query));
            }
            ContentArea::Settings => {
                commands.spawn(settings_content_area(parent_query));
            }
        }
    }
}
/// This function is used to clean up the content area when the state changes
/// It despawns the content area entity
/// and all its children. This is done to avoid having multiple content areas
/// in the same place at the same time.
/// It may be beneficial to instead hide the content area and disable it instead.
fn cleanup_content_area(mut commands: Commands, query: Query<Entity, With<ContentArea>>) {
    if let Ok(query) = query.single() {
        commands.entity(query).despawn();
    }
}
