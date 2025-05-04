use bevy::prelude::*;

use super::{
    base::LauncherIconImages,
    styles::{
        LG_GAP, LG_PADDING, MD_PADDING, MD_ROUNDING, PRIMARY_COLOR, SECONDARY_COLOR, SM_ROUNDING,
        XL_GAP, icon, md_font_center,
    },
};

fn projects_button(text: String, image_handle: Handle<Image>) -> impl Bundle {
    (
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(36.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                column_gap: XL_GAP,
                padding: MD_PADDING,
                ..default()
            },
            BackgroundColor(Color::hsla(0.0, 0.0, 0.16, 1.0)),
            SM_ROUNDING,
            SideBarButton::Project,
        ),
        children![
            icon(image_handle, (Val::Px(16.0), Val::Px(16.0))),
            md_font_center(text),
        ],
    )
}

fn templates_button(text: String, image_handle: Handle<Image>) -> impl Bundle {
    (
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(36.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                column_gap: XL_GAP,
                padding: MD_PADDING,
                ..default()
            },
            BackgroundColor(PRIMARY_COLOR),
            SM_ROUNDING,
            SideBarButton::Templates,
        ),
        children![
            icon(image_handle, (Val::Px(16.0), Val::Px(16.0))),
            md_font_center(text),
        ],
    )
}

fn installs_button(text: String, image_handle: Handle<Image>) -> impl Bundle {
    (
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(36.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                column_gap: XL_GAP,
                padding: MD_PADDING,
                ..default()
            },
            BackgroundColor(PRIMARY_COLOR),
            SM_ROUNDING,
            SideBarButton::Installs,
        ),
        children![
            icon(image_handle, (Val::Px(16.0), Val::Px(16.0))),
            md_font_center(text),
        ],
    )
}

fn assets_button(text: String, image_handle: Handle<Image>) -> impl Bundle {
    (
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(36.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                column_gap: XL_GAP,
                padding: MD_PADDING,
                ..default()
            },
            BackgroundColor(PRIMARY_COLOR),
            SM_ROUNDING,
            SideBarButton::Assets,
        ),
        children![
            icon(image_handle, (Val::Px(16.0), Val::Px(16.0))),
            md_font_center(text),
        ],
    )
}

fn learn_button(text: String, image_handle: Handle<Image>) -> impl Bundle {
    (
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(36.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                column_gap: XL_GAP,
                padding: MD_PADDING,
                ..default()
            },
            BackgroundColor(PRIMARY_COLOR),
            SM_ROUNDING,
            SideBarButton::Learn,
        ),
        children![
            icon(image_handle, (Val::Px(16.0), Val::Px(16.0))),
            md_font_center(text),
        ],
    )
}

fn community_button(text: String, image_handle: Handle<Image>) -> impl Bundle {
    (
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(36.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                column_gap: XL_GAP,
                padding: MD_PADDING,
                ..default()
            },
            BackgroundColor(PRIMARY_COLOR),
            SM_ROUNDING,
            SideBarButton::Community,
        ),
        children![
            icon(image_handle, (Val::Px(16.0), Val::Px(16.0))),
            md_font_center(text),
        ],
    )
}

fn donate_button(text: String, image_handle: Handle<Image>) -> impl Bundle {
    (
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(36.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                column_gap: XL_GAP,
                padding: MD_PADDING,
                ..default()
            },
            BackgroundColor(PRIMARY_COLOR),
            SM_ROUNDING,
            SideBarButton::Donate,
        ),
        children![
            icon(image_handle, (Val::Px(16.0), Val::Px(16.0))),
            md_font_center(text),
        ],
    )
}

pub(crate) fn side_bar(images: LauncherIconImages) -> impl Bundle {
    (
        (
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                padding: MD_PADDING,
                row_gap: LG_GAP,

                ..default()
            },
            MD_ROUNDING,
            BackgroundColor(PRIMARY_COLOR),
        ),
        children![
            projects_button("Projects".to_string(), images.projects_icon.clone()),
            templates_button("Templates".to_string(), images.templates_icon.clone()),
            installs_button("Installs".to_string(), images.installs_icon.clone()),
            assets_button("Assets".to_string(), images.assets_icon.clone()),
            learn_button("Learn".to_string(), images.learn_icon.clone()),
            community_button("Community".to_string(), images.community_icon.clone()),
            donate_button("Donate".to_string(), images.donate_icon.clone()),
            side_bar_bottom(images),
        ],
    )
}

#[derive(Component)]
pub(crate) enum SideBarButton {
    Project,
    Templates,
    Installs,
    Assets,
    Learn,
    Community,
    Donate,
    Settings,
}

fn side_bar_bottom(images: LauncherIconImages) -> impl Bundle {
    (
        (
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                width: Val::Auto,
                height: Val::Percent(100.0),
                padding: LG_PADDING,
                row_gap: LG_GAP,
                ..default()
            },
            BackgroundColor(PRIMARY_COLOR),
        ),
        children![
            settings_button(images.settings_icon.clone()),
            socials(images)
        ],
    )
}

fn settings_button(settings_icon: Handle<Image>) -> impl Bundle {
    (
        (
            Node {
                width: Val::Auto,
                height: Val::Auto,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: XL_GAP,
                padding: MD_PADDING,
                border: UiRect::bottom(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(PRIMARY_COLOR),
            MD_ROUNDING,
            BorderColor(SECONDARY_COLOR),
            SideBarButton::Settings,
        ),
        children![
            icon(settings_icon, (Val::Px(16.0), Val::Px(16.0))),
            md_font_center("Settings".to_string())
        ],
    )
}

fn socials(images: LauncherIconImages) -> impl Bundle {
    (
        (
            Node {
                width: Val::Auto,
                height: Val::Auto,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: LG_GAP * 2.0,
                padding: MD_PADDING,
                ..default()
            },
            BackgroundColor(PRIMARY_COLOR),
        ),
        children![
            (
                icon(images.website_icon, (Val::Px(16.0), Val::Px(16.0))),
                SocialLinks::Website
            ),
            (
                icon(images.github_icon, (Val::Px(16.0), Val::Px(16.0))),
                SocialLinks::Github
            ),
            (
                icon(images.discord_icon, (Val::Px(17.0), Val::Px(13.0))),
                SocialLinks::Discord
            ),
            (
                icon(images.bluesky_icon, (Val::Px(16.0), Val::Px(13.0))),
                SocialLinks::Bluesky
            ),
            (
                icon(images.mastadon_icon, (Val::Px(16.0), Val::Px(16.0))),
                SocialLinks::Mastodon
            ),
            (
                icon(images.reddit_icon, (Val::Px(15.0), Val::Px(16.0))),
                SocialLinks::Reddit
            ),
        ],
    )
}

#[derive(Component)]
pub enum SocialLinks {
    Website,
    Github,
    Discord,
    Bluesky,
    Mastodon,
    Reddit,
}

pub fn socials_observer(trigger: Trigger<Pointer<Click>>, social_buttons: Query<&SocialLinks>) {
    match social_buttons.get(trigger.target()).unwrap() {
        SocialLinks::Website => {
            let _ = open::that_detached("https://bevyengine.org/");
        }
        SocialLinks::Github => {
            let _ = open::that_detached("https://github.com/bevyengine/bevy");
        }
        SocialLinks::Bluesky => {
            let _ = open::that_detached("https://bsky.app/profile/bevyengine.org");
        }
        SocialLinks::Discord => {
            let _ = open::that_detached("https://discord.gg/bevy");
        }
        SocialLinks::Mastodon => {
            let _ = open::that_detached("https://mastodon.social/@bevy");
        }
        SocialLinks::Reddit => {
            let _ = open::that_detached("https://www.reddit.com/r/bevy/");
        }
    }
}
