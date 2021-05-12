use bevy::prelude::*;

use nakama_rs::{api_client::ApiClient, config};

pub use nakama_rs::*;

#[derive(Clone, Debug)]
pub struct NakamaConfig {
    pub key: String,
    pub server: String,
    pub port: u32,
    pub protocol: String,
}

impl Default for NakamaConfig {
    fn default() -> Self {
        Self {
            key: "defaultkey".into(),
            server: "127.0.0.1".into(),
            port: config::DEFAULT_PORT,
            protocol: "http".into(),
        }
    }
}

fn setup(config: Res<Option<NakamaConfig>>, mut client: NonSendMut<Option<ApiClient>>) {
    if let Some(config) = &*config {
        let api_client = ApiClient::new(&config.key, &config.server, config.port, &config.protocol);
        *client = Some(api_client);
    }
}

fn config_changed(config: Res<Option<NakamaConfig>>, mut client: NonSendMut<Option<ApiClient>>) {
    if config.is_changed() {
        if let Some(config) = &*config {
            let api_client =
                ApiClient::new(&config.key, &config.server, config.port, &config.protocol);
            *client = Some(api_client);
        }
    }
}

fn tick(mut client: NonSendMut<Option<ApiClient>>) {
    if let Some(client) = client.as_mut() {
        client.tick();
    }
}

pub struct NakamaPlugin;

impl Plugin for NakamaPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<Option<NakamaConfig>>()
            .init_non_send_resource::<Option<ApiClient>>()
            .add_startup_system(setup.system())
            .add_system(config_changed.system())
            .add_system(tick.system());
    }
}
