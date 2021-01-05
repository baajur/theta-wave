use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CharacterComponent {
    pub money: usize,
    pub collision_damage: f32,
}

impl Component for CharacterComponent {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct PlayerTag;

impl Component for PlayerTag {
    type Storage = NullStorage<Self>;
}
