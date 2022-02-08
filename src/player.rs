
use bevy::prelude::*;

use crate::animation::*;
use crate::resources::*;

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
}


#[derive(PartialEq,Eq,Copy,Clone,Debug)]
pub enum PlayerState {
    Standing(Dir),
    Walking(Dir)
}

impl Player {
    pub fn new() -> Self {
        Self {
            state: PlayerState::Standing(Dir::S),
        }
    }

    pub fn update_state(
        &mut self,
        state: PlayerState,
        animation: &mut Animation,
        animations: &AnimationMap
    ) {
        let ga = |name: &AnimationName| {
            animations.0.get(name).unwrap().clone()
        };
        if self.state != state {
            self.state = state;
            match state {
                PlayerState::Standing(dir) => *animation = ga(&AnimationName::Standing(dir)),
                PlayerState::Walking(dir) => *animation = ga(&AnimationName::Walking(dir))
            }
        }
    }
}
