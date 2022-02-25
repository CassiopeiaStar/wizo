use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;


#[derive(Bundle)]
pub struct BushBundle {
    pub sprite: TextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub blockbox: BlockBox,
    pub hitbox: HitBox,
    pub height: Height,
}

impl BushBundle {
    pub fn new(atlas_map: &AtlasMap,transform: Transform) -> Self {
        Self {
            transform,
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            texture_atlas: atlas_map.get(&AtlasName::Bush),
            global_transform: Default::default(),
            visibility: Default::default(),
            blockbox: BlockBox(
                CollisionRect::centered(Vec2::ZERO,Vec2::new(10.,10.))
            ),
            hitbox: HitBox(
                CollisionRect::centered(Vec2::ZERO,Vec2::new(10.,10.))
            ),
            height: Height(0.),
        }
    }
}


#[derive(Bundle)]
pub struct TreeBundle {
    pub sprite: TextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub blockbox: BlockBox,
    pub hitbox: HitBox,
    pub height: Height,
}

impl TreeBundle {
    pub fn new(atlas_map: &AtlasMap,mut transform: Transform) -> Self {
        transform.translation.y+=16.;
        Self {
            transform,
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            texture_atlas: atlas_map.get(&AtlasName::Tree),
            global_transform: Default::default(),
            visibility: Default::default(),
            blockbox: BlockBox(
                CollisionRect::centered(Vec2::Y*-16.,Vec2::new(12.,10.))
            ),
            hitbox: HitBox(
                CollisionRect::centered(Vec2::Y*-16.,Vec2::new(12.,10.))
            ),
            height: Height(16.),
        }
    }
}


#[derive(Bundle)]
pub struct SignBundle {
    pub sprite: TextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub blockbox: BlockBox,
    pub hitbox: HitBox,
    pub height: Height,
    pub sign: Sign,
}

impl SignBundle {
    pub fn new(atlas_map: &AtlasMap,mut transform: Transform,text:String) -> Self {
        transform.translation.y+=8.;
        Self {
            transform,
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            texture_atlas: atlas_map.get(&AtlasName::Sign),
            global_transform: Default::default(),
            visibility: Default::default(),
            blockbox: BlockBox(
                CollisionRect::centered(Vec2::Y*-8.,Vec2::new(4.,4.))
            ),
            hitbox: HitBox(
                CollisionRect::centered(Vec2::Y*-8.,Vec2::new(4.,4.))
            ),
            height: Height(8.),
            sign: Sign(text),
        }
    }
}
