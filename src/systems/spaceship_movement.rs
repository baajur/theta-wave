use crate::components::{
    Hitbox2DComponent, Motion2DComponent, PlayerTag, Rigidbody, SpaceshipComponent,
};
use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub struct SpaceshipMovementSystem;

impl<'s> System<'s> for SpaceshipMovementSystem {
    type SystemData = (
        ReadStorage<'s, PlayerTag>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpaceshipComponent>,
        WriteStorage<'s, Motion2DComponent>,
        ReadStorage<'s, Hitbox2DComponent>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            player_tags,
            mut transforms,
            mut spaceships,
            mut motion_2d_components,
            hitboxes,
            input,
            time,
        ): Self::SystemData,
    ) {
        let x_move = input.axis_value("player_x").unwrap() as f32;
        let y_move = input.axis_value("player_y").unwrap() as f32;

        for (_player_tag, spaceship, transform, motion_2d, hitbox) in (
            &player_tags,
            &mut spaceships,
            &mut transforms,
            &mut motion_2d_components,
            &hitboxes,
        )
            .join()
        {
            //keep spaceship with bounds of arena
            spaceship.constrain_to_arena(transform, motion_2d, hitbox);

            spaceship.accelerate(x_move, y_move, motion_2d);

            spaceship.update_position(transform, time.delta_seconds(), motion_2d);
        }
    }
}
