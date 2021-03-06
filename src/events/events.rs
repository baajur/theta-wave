use amethyst::{audio::SourceHandle, core::math::Vector2, ecs::prelude::Entity};
use std::collections::HashMap;

#[derive(Debug)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

impl CollisionEvent {
    pub fn new(entity_a: Entity, entity_b: Entity) -> CollisionEvent {
        CollisionEvent { entity_a, entity_b }
    }
}

#[derive(Debug)]
pub struct EnemyDestroyedEvent {
    pub enemy: Entity,
}

impl EnemyDestroyedEvent {
    pub fn new(enemy: Entity) -> EnemyDestroyedEvent {
        EnemyDestroyedEvent { enemy }
    }
}

#[derive(Debug)]
pub struct PlayerCollisionEvent {
    pub player_entity: Entity,
    pub colliding_entity: Entity,
    pub collision_velocity: Option<Vector2<f32>>,
}

impl PlayerCollisionEvent {
    pub fn new(
        entity_a: Entity,
        entity_b: Entity,
        velocity: Option<Vector2<f32>>,
    ) -> PlayerCollisionEvent {
        PlayerCollisionEvent {
            player_entity: entity_a,
            colliding_entity: entity_b,
            collision_velocity: velocity,
        }
    }
}

#[derive(Debug)]
pub struct EnemyCollisionEvent {
    pub enemy_entity: Entity,
    pub colliding_entity: Entity,
    pub collision_velocity: Option<Vector2<f32>>,
}

impl EnemyCollisionEvent {
    pub fn new(
        entity_a: Entity,
        entity_b: Entity,
        velocity: Option<Vector2<f32>>,
    ) -> EnemyCollisionEvent {
        EnemyCollisionEvent {
            enemy_entity: entity_a,
            colliding_entity: entity_b,
            collision_velocity: velocity,
        }
    }
}

#[derive(Debug)]
pub struct ItemEffectGetEvent {
    pub stat_effects: HashMap<String, f32>,
}

impl ItemEffectGetEvent {
    pub fn new(stat_effects: HashMap<String, f32>) -> ItemEffectGetEvent {
        ItemEffectGetEvent { stat_effects }
    }
}

#[derive(Debug)]
pub struct EnemyReachedBottomEvent {
    pub damage: f32,
}

impl EnemyReachedBottomEvent {
    pub fn new(damage: f32) -> EnemyReachedBottomEvent {
        EnemyReachedBottomEvent { damage }
    }
}

#[derive(Debug)]
pub struct PlayAudioEvent {
    pub source: SourceHandle,
}
