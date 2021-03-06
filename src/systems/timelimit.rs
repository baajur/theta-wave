use crate::components::TimeLimitComponent;
use amethyst::{
    core::timing::Time,
    ecs::prelude::{Entities, Join, Read, System, WriteStorage},
};

pub struct TimeLimitSystem;

impl<'s> System<'s> for TimeLimitSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, TimeLimitComponent>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut timelimits, time): Self::SystemData) {
        for (timed_entity, time_component) in (&*entities, &mut timelimits).join() {
            if time_component.duration > 0.0 {
                time_component.duration -= time.delta_seconds();
            } else {
                entities
                    .delete(timed_entity)
                    .expect("unable to delete entity");
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use amethyst::{
        ecs::prelude::{Builder, Entity, WorldExt},
        Error,
    };
    use amethyst_test::prelude::*;

    use crate::components::TimeLimitComponent;

    #[test]
    fn test_timelimit_system() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(TimeLimitSystem, "timelimit_system", &[])
            .with_effect(|world| {
                let entity = world
                    .create_entity()
                    .with(TimeLimitComponent { duration: -1.0 })
                    .build();
                world.insert(EffectReturn(entity));
            })
            .with_assertion(|world| {
                let entity = world.read_resource::<EffectReturn<Entity>>().0.clone();
                world.maintain();
                assert!(!world.is_alive(entity));
            })
            .run()
    }
}
