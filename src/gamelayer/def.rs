use avian2d::prelude::*;

#[derive(PhysicsLayer, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameLayer {
    #[default]
    Player,
    Ground,
}