use bevy::prelude::*;
use avian2d::prelude::*;

use crate::player::state_machine::{handle_grounded, handle_jump_buffer, handle_states};

pub enum PlayerState {
    Idle,
    Walk,
    Fall, 
    Jump
}

impl Player {
    pub fn change_state(&mut self, state: PlayerState) {
        self.state = state;
    }
}

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,

    pub jump_buffer: Timer,
    pub jump_buffer_active: bool,
    pub jump_strength: f32,
    
    pub coyote_timer: Timer,

    pub jump_exhaust: Timer,

    pub movement_speed: f32,

    pub is_grounded: bool,
    pub grounded_ray_origin_relative: Vec2, 
    pub grounded_ray_distance: f32,

}

impl Default for Player {
    fn default() -> Self {
        Player {
            jump_buffer: Timer::from_seconds(0.15, TimerMode::Once),
            jump_exhaust: Timer::from_seconds(2.0, TimerMode::Repeating),
            jump_strength: 600.0,
            jump_buffer_active: false,

            state: PlayerState::Walk,

            is_grounded: false,
            grounded_ray_distance: 1.0,
            grounded_ray_origin_relative: Vec2::new(0.0, -93.0 / 2.0),

            movement_speed: 450.0,


            coyote_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            (handle_grounded, handle_jump_buffer, handle_states).chain()
    );
    }
}


pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let player_sprite: Handle<Image> = asset_server.load("textures/player.png");
    commands.spawn((
    Player::default(),
    Sprite::from_image(player_sprite),
    Transform::from_translation(Vec3::new(2.0, 2.0, 2.0)),
    Collider::rectangle(78.0, 93.0),
    RigidBody::Dynamic,
    LockedAxes::ROTATION_LOCKED,
    GravityScale(1.0),
    )
    );
}   














