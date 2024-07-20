
use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Playing,
    GameOver
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            // Uncomment this to override the default log settings:
            level: bevy::log::Level::DEBUG,
            // filter: "wgpu=warn,bevy_ecs=info".to_string(),
            ..default()
        }))
        .init_state::<GameState>()
        .run();
}
