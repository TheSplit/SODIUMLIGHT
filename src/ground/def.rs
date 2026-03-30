use bevy::{color::palettes::css::PURPLE, prelude::*};
use avian2d::prelude::*;
use crate::gamelayer::def::GameLayer;

pub fn spawn_ground(

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,

) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(1920.0, 200.0)))),
        MeshMaterial2d(materials.add(Color::Srgba(PURPLE))),
        Transform::from_translation(Vec3::new(0.0, -400.0, 0.0)),
        Collider::rectangle(1920.0, 200.0),
        RigidBody::Static,
        CollisionLayers::new(GameLayer::Ground, LayerMask::ALL),
        
    ));
}