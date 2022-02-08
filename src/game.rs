
use bevy::prelude::*;

use crate::GameState;
use crate::input::*;
use crate::animation::{Frame,animation_system,Animation};
use crate::player::*;
use crate::movement::*;
use crate::resources::*;
use crate::components::*;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
struct PreUpdate;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(InputState::default())
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Game)
                .label(PreUpdate)
                .with_system(game_input)
                .with_system(animation_system)
            )
            .add_system_set(SystemSet::on_update(GameState::Game)
                .after(PreUpdate)
                .with_system(move_player)
                .with_system(movement_system)
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(cleanup));
    }
}

struct GameData {
    entities: Vec<Entity>,
}



fn setup(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let atlas_map = AtlasMap::load(&asset_server,&mut texture_atlases);
    let animations = AnimationMap::load(&atlas_map);


    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());

    let scale = Vec3::new(5.,5.,1.);

    let player_ent = cmd.spawn_bundle(SpriteSheetBundle {
        texture_atlas: atlas_map.get(&AtlasName::WalkingDown),
        sprite: TextureAtlasSprite{
            index: 0,
            //custom_size: Some((1.,1.).into()),
            ..Default::default()
        },
        transform:Transform {
            translation: Vec3::new(0.,0.,1.),
            scale,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player::new())
    .insert(Animation::new((0 as usize..=0).map(|i|{
        Frame{
            duration:0.1,
            sprite: Some(TextureAtlasSprite {
                index:i,
                ..Default::default()
            }),
            atlas: Some(atlas_map.get(&AtlasName::WalkingDown))
        }
    }).collect()))
    .insert(MovementBox(CollisionRect{
        pos: Vec2::new(-4.,-16.),
        size: Vec2::new(8.,8.)
    }))
    .insert(Velocity(Vec2::ZERO))
    .id();

    cmd.spawn_bundle(SpriteSheetBundle {
        texture_atlas: atlas_map.get(&AtlasName::Bush),
        sprite: TextureAtlasSprite{
            index:0,
            ..Default::default()
        },
        transform:Transform{
            translation: Vec3::new(-300.,0.,0.),
            scale,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(BlockBox(CollisionRect::centered(Vec2::ZERO,Vec2::new(10.,10.))))
    ;


    let mut game_data = GameData {
        entities: vec![],
    };

    game_data.entities.push(player_ent);

    cmd.insert_resource(game_data);
    cmd.insert_resource(animations);
    cmd.insert_resource(atlas_map);
}

fn cleanup(
    mut cmd: Commands,
    data: Res<GameData>
) {
    for ent in data.entities.iter() {
        //cmd.entity(*ent).despawn_recursive();
    }
    //cmd.remove_resource::<GameData>();
}

fn move_player(
    //data: Res<GameData>
    mut query: Query<(&mut Velocity,&mut Player,&mut Animation)>,
    time: Res<Time>,
    input_state: Res<InputState>,
    animations: Res<AnimationMap>,
) {
    let mut left = input_state.left;
    let mut right = input_state.right;
    let mut up = input_state.up;
    let mut down = input_state.down;
    if left && right {left = false; right = false;}
    if up && down {up = false; down = false;}
    let speed = 3.;
    for (mut vel,mut player,mut animation) in query.iter_mut() {
        if left && up {
            vel.0.x = -speed*0.707;
            vel.0.y = speed*0.707;
            player.update_state(PlayerState::Walking(Dir::W),&mut animation,&animations);
        } else if left && down {
            vel.0.x = -speed*0.707;
            vel.0.y = -speed*0.707;
            player.update_state(PlayerState::Walking(Dir::W),&mut animation,&animations);
        } else if right && up {
            vel.0.x = speed*0.707;
            vel.0.y = speed*0.707;
            player.update_state(PlayerState::Walking(Dir::E),&mut animation,&animations);
        } else if right && down {
            vel.0.x = speed*0.707;
            vel.0.y = -speed*0.707;
            player.update_state(PlayerState::Walking(Dir::E),&mut animation,&animations);
        } else if left {
            vel.0.x = -speed;
            vel.0.y = 0.;
            player.update_state(PlayerState::Walking(Dir::W),&mut animation,&animations);
        } else if right {
            vel.0.x = speed;
            vel.0.y = 0.;
            player.update_state(PlayerState::Walking(Dir::E),&mut animation,&animations);
        } else if up {
            vel.0.y = speed;
            vel.0.x = 0.;
            player.update_state(PlayerState::Walking(Dir::N),&mut animation,&animations);
        } else if down {
            vel.0.y = -speed;
            vel.0.x = 0.;
            player.update_state(PlayerState::Walking(Dir::S),&mut animation,&animations);
        } else {
            vel.0.x = 0.;
            vel.0.y = 0.;
            let dir = {
                match player.state {
                    PlayerState::Walking(dir) => dir,
                    PlayerState::Standing(dir) => dir,
                }
            };
            player.update_state(PlayerState::Standing(dir),&mut animation,&animations);
        }
    }
}

fn player_attack_system(
    mut cmd: Commands,
    input_state: Res<InputState>,
    player: Query<Entity,With<Player>>,
){
    let player_ent = player.single();
    if input_state.attack {
        //cmd.spawn_bundle()
    }
}
