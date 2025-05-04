use bevy::prelude::*;

use crate::launcher_ui::{base::MainArea, styles::{
    lg_font_center, xl_font_center, xsm_font_center, GAP, HIGHLIGHT_COLOR, MD_PADDING, MD_ROUNDING, PRIMARY_COLOR, SECONDARY_COLOR, XL_GAP
}};

pub struct SystemCheckPlugin;

impl Plugin for SystemCheckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, install_rustup_ui.run_if(check_for_rustup));
    }
}

/// Check if Rustup is installed
/// If not, show a popup asking if the user wants to install it
/// If the user chooses to install it, run the install_rustup function
/// If the user chooses to ignore it, systems reliant on Rustup need to be disabled
/// until the user installs it.
fn check_for_rustup() -> bool {
    if std::process::Command::new("rustup")
        .arg("-V")
        .output()
        .is_err()
    {
        error!("Rustup is not installed. Please install it from https://rustup.rs/");
        true
    } else {
        false
    }
}

/* fn install_rustup() {
    //Do nothing for now
    if cfg!(target_os = "linux") {
        let _ = std::process::Command::new("curl")
            .arg("--proto")
            .arg("`=https`")
            .arg("--tlsv1.2")
            .arg("-sSf")
            .arg("https://sh.rustup.rs")
            .arg("|")
            .arg("sh")
            .spawn();
    } else if cfg!(target_os = "windows") {
    } 
} */

/* fn check_for_cargo() -> bool {
    if std::process::Command::new("cargo")
        .arg("-V")
        .output()
        .is_err()
    {
        error!("Cargo is not installed. Please install it from https://rustup.rs/");
        false
    } else {
        true
    }
} */

fn install_rustup_ui(mut commands: Commands, main_area: Query<Entity, With<MainArea>>) {
    let main_area = main_area.single();
    let rust_popup = commands.spawn(modal()).id();
    commands.entity(main_area.unwrap()).add_child(rust_popup);
    
}

pub(crate) fn modal() -> impl Bundle {
    (
        (
            Node {
                display: Display::Flex,
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::End,
                align_items: AlignItems::End,
                ..Default::default()
            },
            GlobalZIndex(1000),
            Pickable {
                should_block_lower: false,
                ..Default::default()
            },
        ),
        children![(
            Node {
                width: Val::Auto,
                height: Val::Auto,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: MD_PADDING,
                border: UiRect::all(Val::Px(1.0)),
                row_gap: XL_GAP,
                ..Default::default()
            },
            BackgroundColor(PRIMARY_COLOR),
            BorderColor(SECONDARY_COLOR),
            BoxShadow(vec![ShadowStyle {
                color: Color::hsla(0.0, 0.0, 0.0, 0.25),
                x_offset: Val::Px(-4.0),
                y_offset: Val::Px(-4.0),
                spread_radius: Val::Px(56.0),
                blur_radius: Val::Px(17.0),
            }]),
            BoxShadowSamples(10),
            MD_ROUNDING,
            
            children![
                (Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: MD_PADDING,
                    border: UiRect::bottom(Val::Px(1.0)),
                    ..Default::default()
                }, BorderColor(SECONDARY_COLOR),
                children![xl_font_center("Rust Not Detected!".to_string(), Color::hsla(0.0, 1.0, 0.32, 1.0))]),
                xsm_font_center(
                    "Without Rust and Cargo, the launcher cannot create new bevy projects, or help you manage rust versions.\nIf you do have it installed please make sure the launcher has permissions to run their commands on your system.\nWould you like for the Launcher to install them for you or proceed with limited capabilities?".to_string()
                ),
                (Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: MD_PADDING,
                    column_gap: GAP,
                    ..Default::default()
                }, 
                children![(
                        Node {
                            width: Val::Auto,
                            height: Val::Auto,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: MD_PADDING,
                            ..Default::default()
                        }, 
                        BackgroundColor(HIGHLIGHT_COLOR),
                        MD_ROUNDING,
                        children![lg_font_center("Install Via Rustup".to_string())]
                    ),
                    (
                        Node {
                            width: Val::Auto,
                            height: Val::Auto,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: MD_PADDING,
                            ..Default::default()
                        }, 
                        BackgroundColor(SECONDARY_COLOR), MD_ROUNDING,
                        children![lg_font_center("Ignore".to_string())]
                )])
                
                
            ]
        ),],
    )
}
