use bevy::prelude::*;

/// Represents a project.
/// TODO: Integrate with Bevy_cli to create/manage projects.
/// TODO: Save projects to a list in the launcher settings file.
/// TODO: Impl recovery if a project is listed but not found.
/// TODO: Impl recovery if a project is found but not listed.
#[derive(Component)]
#[require(
    ProjectName,
    ProjectDescription,
    ProjectVersion,
    ProjectAuthor,
    ProjectPath
)]
pub struct ProjectItem;

#[derive(Component, Default)]
struct ProjectName(String);

#[derive(Component, Default)]
struct ProjectDescription(String);

#[derive(Component, Default)]
struct ProjectVersion(String);

#[derive(Component, Default)]
struct ProjectAuthor(String);

#[derive(Component, Default)]
struct ProjectPath(String);
