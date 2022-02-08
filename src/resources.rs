

use bevy::prelude::*;
use crate::animation::*;
use std::collections::HashMap;

#[derive(Hash,PartialEq,Eq,Copy,Clone,Debug)]
pub enum Dir {
    N,E,S,W
}

#[derive(Hash,Eq,PartialEq,Copy,Clone)]
pub enum AtlasName {
    WalkingUp,
    WalkingDown,
    WalkingSide,
    Bush,
    Sword,
    NoTex,
}

pub struct AtlasMap(pub HashMap<AtlasName,Handle<TextureAtlas>>);

impl AtlasMap {
    pub fn load(
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> AtlasMap {
        let mut hash_map = HashMap::new();
        let mut load_texture = |
            atlas_name: AtlasName,
            path: &str,
            size: Vec2,
            col: usize,
            rows: usize
        | {
            let texture_handle: Handle<Image> = asset_server.load(path);
            let texture_atlas = TextureAtlas::from_grid(texture_handle, size,col,rows);
            hash_map.insert(atlas_name,texture_atlases.add(texture_atlas));
        };


        use AtlasName::*;
        load_texture(WalkingDown,
                     "sprites/walking-down.png",
                     Vec2::new(16.,32.),6,1);
        load_texture(WalkingUp,
                     "sprites/walking-up.png",
                     Vec2::new(16.,32.),6,1);
        load_texture(WalkingSide,
                     "sprites/walking-side.png",
                     Vec2::new(16.,32.),6,1);
        load_texture(Bush,
                     "sprites/bush.png",
                     Vec2::new(16.,16.),1,1);
        load_texture(Sword,
                     "sprites/sword.png",
                     Vec2::new(32.,32.),3,1);
        load_texture(NoTex,
                     "sprites/sword.png",
                     Vec2::new(16.,16.),1,1);


        AtlasMap(hash_map)
    }

    pub fn get(&self, key: &AtlasName) -> Handle<TextureAtlas>{
        self.0.get(key).unwrap_or(self.0.get(&AtlasName::NoTex).unwrap()).clone()
    }
}


#[derive(Hash,Eq,PartialEq,Copy,Clone)]
pub enum AnimationName {
    Standing(Dir),
    Walking(Dir),
}

pub struct AnimationMap(pub HashMap<AnimationName,Animation>);

impl AnimationMap {
    pub fn load(atlas_map: &AtlasMap) -> Self {
        let mut hash_map = HashMap::new();

        //get_atlas
        let ga = |atlas_name| {
            Some(atlas_map.0.get(&atlas_name).unwrap().clone())
        };

        use AtlasName::*;
        hash_map.insert(AnimationName::Standing(Dir::E),
        Animation::new(vec![Frame{
            duration: 100.,
            sprite: Some(TextureAtlasSprite {
                index: 0,
                ..Default::default()
            }),
            atlas: ga(WalkingSide)
        }]));
        hash_map.insert(AnimationName::Standing(Dir::W),
        Animation::new(vec![Frame{
            duration: 100.,
            sprite: Some(TextureAtlasSprite {
                index: 0,
                flip_x: true,
                ..Default::default()
            }),
            atlas: ga(WalkingSide)
        }]));
        hash_map.insert(AnimationName::Standing(Dir::S),
        Animation::new(vec![Frame{
            duration: 100.,
            sprite: Some(TextureAtlasSprite {
                index: 0,
                ..Default::default()
            }),
            atlas: ga(WalkingDown)
        }]));
        hash_map.insert(AnimationName::Standing(Dir::N),
        Animation::new(vec![Frame{
            duration: 100.,
            sprite: Some(TextureAtlasSprite {
                index: 0,
                ..Default::default()
            }),
            atlas: ga(WalkingUp)
        }]));
        hash_map.insert(AnimationName::Walking(Dir::N),
        Animation::new((0 as usize..=5).map(|i|{
            Frame{
                duration:0.1,
                sprite: Some(TextureAtlasSprite{
                    index:i,
                    ..Default::default()
                }),
                atlas: ga(WalkingUp)
            }
        }).collect()));
        hash_map.insert(AnimationName::Walking(Dir::S),
        Animation::new((0 as usize..=5).map(|i|{
            Frame{
                duration:0.1,
                sprite: Some(TextureAtlasSprite{
                    index:i,
                    ..Default::default()
                }),
                atlas: ga(WalkingDown)

            }
        }).collect()));
        hash_map.insert(AnimationName::Walking(Dir::E),
        Animation::new((0 as usize..=5).map(|i|{
            Frame{
                duration:0.1,
                sprite: Some(TextureAtlasSprite{
                    index:i,
                    ..Default::default()
                }),
                atlas: ga(WalkingSide)
            }
        }).collect()));
        hash_map.insert(AnimationName::Walking(Dir::W),
        Animation::new((0 as usize..=5).map(|i|{
            Frame{
                duration:0.1,
                sprite: Some(TextureAtlasSprite{
                    index:i,
                    flip_x: true,
                    ..Default::default()
                }),
                atlas: ga(WalkingSide)
            }
        }).collect()));
        dbg!();
        AnimationMap(hash_map)
    }
}

/*
struct Animations {
    pub standing_right: Animation,
    pub standing_left: Animation,
    pub standing_up: Animation,
    pub standing_down: Animation,
    pub walking_right: Animation,
    pub walking_down: Animation,
    pub walking_up: Animation,
    pub walking_left: Animation, 
    //pub sword: Animation,
}

impl Animations {
    fn load(
        atlas_map: &AtlasMap
    ) -> Animations {

        //get_atlas
        let ga = |atlas_name| {
            atlas_map.0.get(&atlas_name).unwrap().clone()
        };

        use AtlasName::*;
        Animations{
            standing_right: Animation::new(vec![Frame{
                duration: 100.,
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..Default::default()
                },
                atlas: ga(WalkingSide)
            }]),
            standing_left: Animation::new(vec![Frame{
                duration: 100.,
                sprite: TextureAtlasSprite {
                    index: 0,
                    flip_x: true,
                    ..Default::default()
                },
                atlas: ga(WalkingSide)
            }]),
            standing_down: Animation::new(vec![Frame{
                duration: 100.,
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..Default::default()
                },
                atlas: ga(WalkingDown)
            }]),
            standing_up: Animation::new(vec![Frame{
                duration: 100.,
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..Default::default()
                },
                atlas: ga(WalkingUp)
            }]),
            walking_up: Animation::new((0 as usize..=5).map(|i|{
                Frame{
                    duration:0.1,
                    sprite: TextureAtlasSprite{
                        index:i,
                        ..Default::default()
                    },
                    atlas: ga(WalkingUp)
                }
            }).collect()),
            walking_down: Animation::new((0 as usize..=5).map(|i|{
                Frame{
                    duration:0.1,
                    sprite: TextureAtlasSprite{
                        index:i,
                        ..Default::default()
                    },
                    atlas: ga(WalkingDown)

                }
            }).collect()),
            walking_right: Animation::new((0 as usize..=5).map(|i|{
                Frame{
                    duration:0.1,
                    sprite: TextureAtlasSprite{
                        index:i,
                        ..Default::default()
                    },
                    atlas: ga(WalkingSide)
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
                    atlas: ga(WalkingSide)
                }
            }).collect()),
        }
    }
}
*/
