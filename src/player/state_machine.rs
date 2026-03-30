

use bevy::prelude::*;
use avian2d::prelude::*;
use crate::player::def::{Player, PlayerState, XDirection};
use crate::gamelayer::def::GameLayer;

pub fn handle_grounded(
    spatial_query: SpatialQuery,
    mut query: Query<(&mut Player, &mut Transform)>

) -> () {
    if let Ok((mut player, transform)) = query.single_mut() {
    let origin: Vec2 = transform.translation.truncate() + player.grounded_ray_origin_relative;
    let hit: Option<RayHitData> = spatial_query.cast_ray(origin, Dir2::NEG_Y, player.grounded_ray_distance, true, 
&SpatialQueryFilter::from_mask([GameLayer::Ground])
    );
        player.is_grounded = hit.is_some()
    }

}

pub fn handle_jump_buffer(
    mut player: Single<&mut Player>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    player.jump_buffer.tick(time.delta());

    if keys.just_pressed(KeyCode::KeyZ) {
        player.jump_buffer.reset();
    }

    player.jump_buffer_active = !player.jump_buffer.is_finished();

    if keys.just_released(KeyCode::KeyZ) {
        player.jump_buffer.finish();
    }
}


pub fn handle_x_movement(player: &mut Player, velocity: &mut LinearVelocity, keys: &Res<ButtonInput<KeyCode>>, time: &Res<Time>) {
        let mut move_direction = 0.0;
        let left = keys.pressed(KeyCode::ArrowLeft);
        let right = keys.pressed(KeyCode::ArrowRight);
        let left_rel = keys.just_released(KeyCode::ArrowLeft);
        let right_rel = keys.just_released(KeyCode::ArrowRight);
        

        if left ^ right { // chuje sie wykluczaja
            if left { player.x_primary = XDirection::Left; player.x_interrupt = XDirection::None }
            else if right { player.x_primary = XDirection::Right; player.x_interrupt = XDirection::None }
        }
        else {

            if player.x_primary == XDirection::Left && right { player.x_interrupt = XDirection::Right }
            if player.x_primary == XDirection::Right && left { player.x_interrupt = XDirection::Left }

            if left_rel && player.x_primary == XDirection::Left { player.x_interrupt = player.x_primary }
            if right_rel && player.x_primary == XDirection::Right { player.x_interrupt = player.x_primary }
        }

        if player.x_interrupt != XDirection::None {
            move_direction = player.x_interrupt as i32 as f32;
        }
        else {
            move_direction = player.x_primary as i32 as f32;
        }
        
        velocity.x = move_direction * player.movement_speed;

}


pub fn fall_state(player: &mut Player, velocity: &mut LinearVelocity, gravity_scale: &mut GravityScale, keys: Res<ButtonInput<KeyCode>>, time: Res<Time>) {
    
    // player.coyote_timer.tick(time.delta());

    // if keys.pressed(KeyCode::ArrowDown) {
    //     gravity_scale.0 = 7.0;
    // }

    velocity.y = velocity.y.max(-600.0);

    if player.is_grounded && player.jump_buffer_active {
        player.coyote_timer.reset();
        gravity_scale.0 = 1.0;
        player.change_state(PlayerState::Idle);
    }

    
    handle_x_movement(player, velocity, &keys, &time);
    consider_jump(player, velocity, gravity_scale, &keys, &time);

}


pub fn consider_jump(player: &mut Player, velocity: &mut LinearVelocity, gravity_scale: &mut GravityScale, keys: &Res<ButtonInput<KeyCode>>, time: &Res<Time>) {

    let has_jump_buffer = !player.jump_buffer.is_finished() && player.jump_buffer.elapsed_secs() > 0.0;
    let can_coyote_jump = !player.coyote_timer.is_finished() && player.coyote_timer.elapsed_secs() > 0.0;

    // if has_jump_buffer && (player.is_grounded || can_coyote_jump) 

    if has_jump_buffer && player.is_grounded
    {
        velocity.y = player.jump_strength;
        gravity_scale.0 = 1.0;
        player.change_state(PlayerState::Jump);
    }

}

pub fn consider_drill(player: &mut Player, velocity: &mut LinearVelocity, gravity_scale: &mut GravityScale, keys: &Res<ButtonInput<KeyCode>>, time: &Res<Time>) {
    if keys.pressed(KeyCode::KeyX) {

    }
}

pub fn drill_state() {

}


pub fn handle_states(
    mut query: Query<(&mut Player, &mut LinearVelocity, &mut GravityScale)>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) { 
        if let Ok((mut player, mut velocity, mut gravity_scale)) = query.single_mut() {

        match player.state {
            PlayerState::Walk => { walk_state(player.as_mut(), velocity.as_mut(), gravity_scale.as_mut(), keys, time) } 
            PlayerState::Fall => { fall_state(player.as_mut(), velocity.as_mut(), gravity_scale.as_mut(), keys, time) }
            PlayerState::Jump => { jump_state(player.as_mut(), velocity.as_mut(), gravity_scale.as_mut(), keys, time) }
            PlayerState::Idle => { idle_state(player.as_mut(), velocity.as_mut(), gravity_scale.as_mut(), keys, time) }
        }
    }
}


pub fn idle_state(player: &mut Player, velocity: &mut LinearVelocity, gravity_scale: &mut GravityScale, keys: Res<ButtonInput<KeyCode>>, time: Res<Time>) { 

    handle_x_movement(player, velocity, &keys, &time);
    consider_jump(player, velocity, gravity_scale, &keys, &time);

    if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::ArrowLeft) {
        player.change_state(PlayerState::Walk);
    }
    else if keys.pressed(KeyCode::KeyZ) {
        player.change_state(PlayerState::Jump);
    }
}

pub fn walk_state(player: &mut Player, velocity: &mut LinearVelocity, gravity_scale: &mut GravityScale, keys: Res<ButtonInput<KeyCode>>, time: Res<Time>) {
        
    handle_x_movement(player, velocity, &keys, &time);
    consider_jump(player, velocity, gravity_scale, &keys, &time);

    if keys.pressed(KeyCode::KeyZ) {
        player.change_state(PlayerState::Jump);
    }
}


pub fn jump_state(player: &mut Player, velocity: &mut LinearVelocity, gravity_scale: &mut GravityScale, keys: Res<ButtonInput<KeyCode>>, time: Res<Time>) {

    handle_x_movement(player, velocity, &keys, &time);

    player.jump_exhaust.tick(time.delta());

    if keys.just_released(KeyCode::KeyZ) {
        velocity.y *= 0.4; 
        player.change_state(PlayerState::Fall);
        gravity_scale.0 = 2.0;
        return;
    }

    if player.jump_exhaust.is_finished() || velocity.y <= 0.0 {
        gravity_scale.0 = 2.0;
        player.change_state(PlayerState::Fall);
    }
}

