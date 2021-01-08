use crate::components::{
    AbilityComponent, AnimationComponent, AutoFireComponent, BarrelRoll, BlasterComponent,
    CharacterComponent, ConsumableComponent, EnemyComponent, HealthComponent, Hitbox2DComponent,
    ItemComponent, Motion2DComponent, SpecialAbility,
};
use amethyst::{
    assets::Handle,
    renderer::{palette::Srgba, SpriteSheet},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EnemyEntityData {
    pub animation_component: AnimationComponent,
    pub enemy_component: EnemyComponent,
    pub hitbox_component: Hitbox2DComponent,
    pub blaster_component: Option<BlasterComponent>,
    pub autofire_component: Option<AutoFireComponent>,
    pub motion2d_component: Motion2DComponent,
    pub health_component: HealthComponent,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ThrusterEntityData {
    pub animation_component: AnimationComponent,
    pub y_offset: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemEntityData {
    pub item_component: ItemComponent,
    pub animation_component: Option<AnimationComponent>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConsumableEntityData {
    pub consumable_component: ConsumableComponent,
    pub hitbox_component: Hitbox2DComponent,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PlayerEntityData<T>
where
    T: SpecialAbility,
{
    pub character_component: CharacterComponent,
    pub ability_component: AbilityComponent<T>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpriteSheetData {
    pub image: String,
    pub data: String,
}

pub type EnemiesResource = HashMap<String, EnemyEntityData>;
pub type ThrustersResource = HashMap<String, ThrusterEntityData>;
pub type ItemsResource = HashMap<String, ItemEntityData>;
pub type ConsumablesResource = HashMap<String, ConsumableEntityData>;
pub type JuggernautPlayerResource = PlayerEntityData<BarrelRoll>;
//pub type PlayersResource = HashMap<String, PlayerEntityData<Any>>;

#[derive(Clone)]
pub struct SpriteSheetsResource {
    pub spritesheets: HashMap<String, Handle<SpriteSheet>>,
}

pub type SpriteSheetsConfig = HashMap<String, SpriteSheetData>;
pub type SoundsConfig = HashMap<String, String>;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct DebugLinesConfig {
    pub line_width: f32,
    pub hitbox_color: Srgba,
}
