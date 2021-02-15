use crate::components::{collider::Collider, paddle::Paddle, player::Player};
use bevy::ecs::Bundle;

#[derive(Bundle)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub player: Player,
    pub collider: Collider,
}

impl PaddleBundle {
    pub fn new(player: Player) -> Self {
        Self {
            paddle: Paddle::default(),
            player: player,
            collider: Collider::Paddle,
        }
    }
}
