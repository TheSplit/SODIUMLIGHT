use bevy::prelude::*;
use avian2d::prelude::*;
use bevy::window::{WindowMode, WindowPlugin};

mod player;
mod ground;
mod gamelayer; 

pub fn setup(

    mut commands: Commands,

) {
    commands.spawn(Camera2d::default());
} 

fn main() {
    App::new()
        .add_plugins(PhysicsPlugins::default())
         .insert_resource(Gravity(Vec2::NEG_Y * 1000.0))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                position: WindowPosition::Automatic,
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                visible: true,
                ..default()
            }),
            ..default()
        }))   
    .add_systems(Startup, setup)
    .add_systems(Startup, (player::def::spawn_player, ground::def::spawn_ground))
    .add_plugins(player::def::PlayerPlugin)
        .run();
}
