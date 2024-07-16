use std::f32::consts::PI;

use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Playing,
    GameOver
}

struct Cell {
    coord: IVec2,
    height: f32
}

#[derive(Default)]
struct Player {
    enity: Option<Entity>,
    cell_coord: IVec2,
    move_cooldown: Timer
}

#[derive(Resource, Default)]
struct Game {
    player: Player,
    score: i32,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            // Uncomment this to override the default log settings:
            level: bevy::log::Level::DEBUG,
            // filter: "wgpu=warn,bevy_ecs=info".to_string(),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default()) // Show Frame rate
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>
) {
    debug!("Setup Game Scene !!!");
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }
    );

    // commands.spawn(PbrBundle {
    //     transform: Transform::from_scale((0.01, 0.01, 0.01).into())
    //         * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI))
    //     mesh: assets.load("models/chr_knight.vox"),
    //     material: materials.add(Color::srgb(1.,1.,1.).into()),
    //     ..default()
    // });
}
