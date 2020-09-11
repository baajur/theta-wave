use crate::{
    components::{Hitbox2DComponent, Motion2DComponent, Spaceship},
    constants::{
        ARENA_HEIGHT, ARENA_MIN_X, ARENA_MIN_Y, ARENA_WIDTH, CRIT_BLAST_SPRITE_INDEX,
        POISON_BLAST_SPRITE_INDEX, SPACESHIP_ACCELERATION_X, SPACESHIP_ACCELERATION_Y,
        SPACESHIP_BARREL_COOLDOWN, SPACESHIP_BARREL_DURATION, SPACESHIP_BARREL_SPEED,
        SPACESHIP_BLAST_SPRITE_INDEX, SPACESHIP_COLLISION_DAMAGE, SPACESHIP_CRIT_CHANCE,
        SPACESHIP_DAMAGE, SPACESHIP_DECELERATION_X, SPACESHIP_DECELERATION_Y, SPACESHIP_FIRE_SPEED,
        SPACESHIP_HEALTH, SPACESHIP_HEIGHT, SPACESHIP_HITBOX_HEIGHT, SPACESHIP_HITBOX_WIDTH,
        SPACESHIP_MAX_KNOCKBACK_SPEED, SPACESHIP_MAX_SPEED, SPACESHIP_MONEY, SPACESHIP_WIDTH,
    },
};
use amethyst::{
    assets::Handle,
    core::{math::Vector2, transform::Transform},
    ecs::{World, WorldExt},
    prelude::Builder,
    renderer::{SpriteRender, SpriteSheet, Transparent},
};
use std::collections::HashMap;

pub fn initialize_spaceship(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(
        ARENA_MIN_X + (ARENA_WIDTH / 2.0),
        ARENA_MIN_Y + (ARENA_HEIGHT / 6.0),
        0.9,
    );

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    let mut blast_sprite_indicies = HashMap::new();
    blast_sprite_indicies.insert("normal".to_string(), SPACESHIP_BLAST_SPRITE_INDEX);
    blast_sprite_indicies.insert("crit".to_string(), CRIT_BLAST_SPRITE_INDEX);
    blast_sprite_indicies.insert("poison".to_string(), POISON_BLAST_SPRITE_INDEX);

    let hitbox = Hitbox2DComponent {
        width: SPACESHIP_HITBOX_WIDTH,
        height: SPACESHIP_HITBOX_HEIGHT,
        offset_x: 0.0,
        offset_y: 0.0,
        offset_rotation: 0.0,
    };

    let motion_2d = Motion2DComponent {
        velocity: Vector2::new(0.0, 0.0),
        acceleration: Vector2::new(SPACESHIP_ACCELERATION_X, SPACESHIP_ACCELERATION_Y),
        deceleration: Vector2::new(SPACESHIP_DECELERATION_X, SPACESHIP_DECELERATION_Y),
        angular_velocity: 0.0,
        angular_acceleration: 0.0,
        angular_deceleration: 0.0,
        max_speed: Vector2::new(SPACESHIP_MAX_SPEED, SPACESHIP_MAX_SPEED),
        knockback_max_speed: Vector2::new(
            SPACESHIP_MAX_KNOCKBACK_SPEED,
            SPACESHIP_MAX_KNOCKBACK_SPEED,
        ),
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Spaceship {
            width: SPACESHIP_WIDTH,
            height: SPACESHIP_HEIGHT,
            fire_speed: SPACESHIP_FIRE_SPEED,
            fire_reset_timer: 0.0,
            damage: SPACESHIP_DAMAGE,
            barrel_cooldown: SPACESHIP_BARREL_COOLDOWN,
            barrel_reset_timer: 0.0,
            barrel_speed: SPACESHIP_BARREL_SPEED,
            barrel_action_right: false,
            barrel_action_left: false,
            barrel_duration: SPACESHIP_BARREL_DURATION,
            barrel_action_timer: SPACESHIP_BARREL_DURATION,
            pos_x: local_transform.translation().x,
            pos_y: local_transform.translation().y,
            blast_speed: 160.0,
            max_health: SPACESHIP_HEALTH,
            health: SPACESHIP_HEALTH,
            money: SPACESHIP_MONEY,
            steel_barrel: false,
            blast_count: 1,
            collision_damage: SPACESHIP_COLLISION_DAMAGE,
            crit_chance: SPACESHIP_CRIT_CHANCE,
            poison_chance: 0.0,
            blast_sprite_indicies,
            allied: true,
        })
        .with(hitbox)
        .with(motion_2d)
        .with(local_transform)
        .with(Transparent)
        .build();
}
