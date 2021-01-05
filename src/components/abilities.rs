use crate::components::Motion2DComponent;

use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, Read},
    input::{InputHandler, StringBindings},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AbilityType {
    BarrelRoll {
        is_active_left: bool,
        is_active_right: bool,
        speed: f32,
        steel_barrel: bool,
    },
    Swap,
    None,
}

impl AbilityType {
    fn execute(&mut self, input: &Read<InputHandler<StringBindings>>) -> bool {
        /*
        match self {
            Self::BarrelRoll {
                mut is_active_left,
                mut is_active_right,
                speed,
                steel_barrel,
            } => {
                let barrel_left = input.action_is_down("barrel_left").unwrap();
                let barrel_right = input.action_is_down("barrel_right").unwrap();

                if barrel_left {
                    is_active_left = true;
                    return true;
                } else if barrel_right {
                    is_active_right = true;
                    return true;
                }
            }
            Self::Swap => {}
            Self::None => {}
        };
        false
        */
        let mut result = false;
        *self = match std::mem::replace(self, AbilityType::None) {
            Self::BarrelRoll {
                mut is_active_left,
                mut is_active_right,
                speed,
                steel_barrel,
            } => {
                let barrel_left = input.action_is_down("barrel_left").unwrap();
                let barrel_right = input.action_is_down("barrel_right").unwrap();

                if barrel_left {
                    is_active_left = true;
                    result = true;
                } else if barrel_right {
                    is_active_right = true;
                    result = true;
                }
                Self::BarrelRoll {
                    is_active_left,
                    is_active_right,
                    speed,
                    steel_barrel,
                }
            }
            _ => Self::None,
        };
        result
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AbilityComponent {
    // cooldown ability specific
    pub execute_cooldown: f32, // cooldown_time
    pub execute_timer: f32,    // cooldown_timer
    pub action_cooldown: f32,  // action_time
    pub action_timer: f32,     // action_timer
    pub is_active: bool,

    pub ability_type: AbilityType,
}

impl Component for AbilityComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AbilityComponent {
    // checks for input then executes special ability if input pressed
    pub fn execute(&mut self, input: &Read<InputHandler<StringBindings>>) {
        if self.execute_timer <= 0.0 && !self.is_active && self.ability_type.execute(input) {
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
                //self.ability_type.end_action()
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
