use bevy::prelude::*;
use discord_rich_presence::{
    activity::{Activity, Assets},
    *,
};

/// Plugin to integrate with Discord Rich Presence.
/// This plugin sets the Discord status to "Running" on Startup.
pub struct DiscordPlugin;

impl Plugin for DiscordPlugin {
    fn build(&self, app: &mut App) {
        // The Client ID is surprisingly not a secret, it's safe to have in in code like this.
        let client = DiscordIpcClient::new("1361050357060341852");
        if client.is_ok() {
            let client = client.unwrap();

            app.insert_resource(DiscordClient { client });
            app.add_systems(Startup, startup_discord_status);
        } else {
            error!("Failed to create Discord client");
        }
    }
}

#[derive(Resource)]
struct DiscordClient {
    client: DiscordIpcClient,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct DiscordIPCSet;

/* #[derive(Event)]
pub struct DiscordStatusEvent {
    pub state: Option<&str>,
    pub details: Option<&str>,
    pub large_image: Option<&str>,
    pub large_text: Option<&str>,
    pub small_image: Option<&str>,
    pub small_text: Option<&str>,

} */

/// This system connects to Discord and sets the initial activity status.
fn startup_discord_status(mut client: ResMut<DiscordClient>) {
    if client.client.connect().is_ok() {
        info!("Discord client connected");
    } else {
        error!("Failed to connect to Discord client");
        return;
    }

    let activity_payload = Activity::new()
        .state("Running")
        .details("Bevy Launcher")
        .assets(
            Assets::new()
                .large_image("cover")
                .large_text("Bevy Launcher"),
        );
    client
        .client
        .set_activity(activity_payload)
        .expect("Failed to set activity");
    info!("Discord status set to 'Running'");
}
