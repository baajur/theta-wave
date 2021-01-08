use crate::components::Motion2DComponent;

use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, Read},
    input::{InputHandler, StringBindings},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AbilityType {
    BarrelRoll,
    None,
}

pub trait SpecialAbility {
    fn execute(&mut self, input: &Read<InputHandler<StringBindings>>) -> bool;
    fn end_action(&mut self);
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BarrelRoll {
    pub is_active_left: bool,
    pub is_active_right: bool,
    pub speed: f32,
    pub steel_barrel: bool,
}

impl SpecialAbility for BarrelRoll {
    fn execute(&mut self, input: &Read<InputHandler<StringBindings>>) -> bool {
        let barrel_left = input.action_is_down("barrel_left").unwrap();
        let barrel_right = input.action_is_down("barrel_right").unwrap();

        if barrel_left {
            self.is_active_left = true;
            return true;
        } else if barrel_right {
            self.is_active_right = true;
            return true;
        }

        false
    }

    fn end_action(&mut self) {
        self.is_active_left = false;
        self.is_active_right = false;
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AbilityComponent<T>
where
    T: SpecialAbility,
{
    // cooldown ability specific
    pub execute_cooldown: f32, // cooldown_time
    pub execute_timer: f32,    // cooldown_timer
    pub action_cooldown: f32,  // action_time
    pub action_timer: f32,     // action_timer
    pub is_active: bool,
    pub ability_type: AbilityType,

    pub special_ability: T,
}

impl<T> Component for AbilityComponent<T> {
    type Storage = DenseVecStorage<Self>;
}

impl<T> AbilityComponent<T>
where
    T: SpecialAbility,
{
    // checks for input then executes special ability if input pressed
    pub fn execute(&mut self, input: &Read<InputHandler<StringBindings>>) {
        if self.execute_timer <= 0.0 && !self.is_active && self.special_ability.execute(input) {
            // reset execution and action cooldowns, and set active
            self.execute_timer = self.execute_cooldown;
            self.action_timer = self.action_cooldown;
            self.is_active = true;
        }
    }

    pub fn update(&mut self, dt: f32) {
        println!("{:?}", self.ability_type);
        if self.is_active {
            self.update_action(dt);
        } else {
            self.update_execution_timer(dt);
        }
    }

    fn update_action(&mut self, dt: f32) {
        if self.action_timer > 0.0 {
            self.action_timer -= dt;

            if self.action_timer <= 0.0 {
                self.is_active = false;
                self.special_ability.end_action()
            }
        }

        //self.ability_type.update_action()
    }

    pub fn update_execution_timer(&mut self, dt: f32) {
        if self.execute_timer > 0.0 {
            self.execute_timer -= dt;
        }
    }
    /*
    pub fn initiate_barrel_roll(&mut self, left: bool, right: bool) {
        if left || right {
            self.action_timer = self.action_cooldown;
            self.execute_timer = self.execute_cooldown;

            if left {
                self.is_active_left = true;
            } else if right {
                self.is_active_right = true;
            }
        }
    }
    */
    /*
    pub fn barrel_input_cooldown(&mut self, dt: f32) -> bool {
        if self.execute_timer > 0.0 && !self.is_active_left && !self.is_active_right {
            self.execute_timer -= dt;
            true
        } else {
            false
        }
    }

    pub fn barrel_action_cooldown(&mut self, dt: f32, motion_2d: &mut Motion2DComponent) -> bool {
        if self.is_active_left || self.is_active_right {
            //update the cooldown
            if self.action_timer > 0.0 {
                self.action_timer -= dt;
            } else {
                if self.is_active_left {
                    motion_2d.velocity.x = -1.0 * motion_2d.max_speed.x;
                }

                if self.is_active_right {
                    motion_2d.velocity.x = motion_2d.max_speed.x;
                }

                self.is_active_left = false;
                self.is_active_right = false;
                self.execute_timer = self.execute_time;
            }

            true
        } else {
            false
        }
    }
    */
}
