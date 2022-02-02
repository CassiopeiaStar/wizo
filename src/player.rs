
use bevy::prelude::*;

use crate::animation::*;

#[derive(Component)]
pub struct Player {
    pub standing_right: Animation,
    pub standing_left: Animation,
    pub standing_up: Animation,
    pub standing_down: Animation,
    pub walking_right: Animation,
    pub walking_down: Animation,
    pub walking_up: Animation,
    pub walking_left: Animation,
    pub state: PlayerState,
}

#[derive(PartialEq,Eq,Copy,Clone,Debug)]
pub enum Dir {
    N,E,S,W
}
#[derive(PartialEq,Eq,Copy,Clone,Debug)]
pub enum PlayerState {
    Standing(Dir),
    Walking(Dir)
}

impl Player {
    pub fn new(
        walking_down: Handle<TextureAtlas>,
        walking_up: Handle<TextureAtlas>,
        walking_side: Handle<TextureAtlas>,
    ) -> Self {
        Self {
            state: PlayerState::Standing(Dir::S),
            standing_right: Animation::new(vec![Frame{
                duration: 100.,
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..Default::default()
                },
                atlas: walking_side.clone()
            }]),
            standing_left: Animation::new(vec![Frame{
                duration: 100.,
                sprite: TextureAtlasSprite {
                    index: 0,
                    flip_x: true,
                    ..Default::default()
                },
                atlas: walking_side.clone()
            }]),
            standing_down: Animation::new(vec![Frame{
                duration: 100.,
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..Default::default()
                },
                atlas: walking_down.clone()
            }]),
            standing_up: Animation::new(vec![Frame{
                duration: 100.,
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..Default::default()
                },
                atlas: walking_up.clone()
            }]),
            walking_up: Animation::new((0 as usize..=5).map(|i|{
                Frame{
                    duration:0.1,
                    sprite: TextureAtlasSprite{
                        index:i,
                        ..Default::default()
                    },
                    atlas: walking_up.clone()
                }
            }).collect()),
            walking_down: Animation::new((0 as usize..=5).map(|i|{
                Frame{
                    duration:0.1,
                    sprite: TextureAtlasSprite{
                        index:i,
                        ..Default::default()
                    },
                    atlas: walking_down.clone()
                }
            }).collect()),
            walking_right: Animation::new((0 as usize..=5).map(|i|{
                Frame{
                    duration:0.1,
                    sprite: TextureAtlasSprite{
                        index:i,
                        ..Default::default()
                    },
                    atlas: walking_side.clone()
                }
            }).collect()),
            walking_left: Animation::new((0 as usize..=5).map(|i|{
                Frame{
                    duration:0.1,
                    sprite: TextureAtlasSprite{
                        index:i,
                        flip_x: true,
                        ..Default::default()
                    },
                    atlas: walking_side.clone()
                }
            }).collect()),
        }
    }

    pub fn update_state(&mut self,state: PlayerState,animation: &mut Animation) {
        if self.state != state {
            self.state = state;
            dbg!(state);
            match state {
                PlayerState::Standing(Dir::N) => *animation = self.standing_up.clone(),
                PlayerState::Standing(Dir::S) => *animation = self.standing_down.clone(),
                PlayerState::Standing(Dir::E) => *animation = self.standing_right.clone(),
                PlayerState::Standing(Dir::W) => *animation = self.standing_left.clone(),
                PlayerState::Walking(Dir::N) => *animation = self.walking_up.clone(),
                PlayerState::Walking(Dir::S) => *animation = self.walking_down.clone(),
                PlayerState::Walking(Dir::E) => *animation = self.walking_right.clone(),
                PlayerState::Walking(Dir::W) => *animation = self.walking_left.clone(),
            }
        }
    }
}
