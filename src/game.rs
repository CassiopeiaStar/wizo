
use bevy::prelude::*;

use crate::GameState;
use crate::input::*;
use crate::animation::{Frame,animation_system,Animation};
use crate::player::*;

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
    let texture_handle = asset_server.load("sprites/walking-down.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.,32.),6,1);
    let walking_down = texture_atlases.add(texture_atlas);

    let texture_handle = asset_server.load("sprites/walking-up.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.,32.),6,1);
    let walking_up = texture_atlases.add(texture_atlas);
    
    let texture_handle = asset_server.load("sprites/walking-side.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.,32.),6,1);
    let walking_side = texture_atlases.add(texture_atlas);

    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());

    let player_ent = cmd.spawn_bundle(SpriteSheetBundle {
        texture_atlas: walking_down.clone(),
        sprite: TextureAtlasSprite{
            index: 0,
            //custom_size: Some((1.,1.).into()),
            ..Default::default()
        },
        transform:Transform {
            translation: Vec3::new(0.,0.,1.),
            scale: Vec3::new(5.,5.,1.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player::new(walking_down.clone(),walking_up.clone(),walking_side.clone()))
    .insert(Animation::new((0 as usize..=0).map(|i|{
        Frame{
            duration:0.1,
            sprite: TextureAtlasSprite {
                index:i,
                ..Default::default()
            },
            atlas: walking_down.clone(),
        }
    }).collect()))

    .id();
    let mut game_data = GameData {
        entities: vec![],
    };

    game_data.entities.push(player_ent);

    cmd.insert_resource(game_data);
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
    mut query: Query<(&mut Transform,&mut Player,&mut Animation)>,
    time: Res<Time>,
    input_state: Res<InputState>,
) {
    let mut left = input_state.left;
    let mut right = input_state.right;
    let mut up = input_state.up;
    let mut down = input_state.down;
    if left && right {left = false; right = false;}
    if up && down {up = false; down = false;}
    let speed = 3.;
    for (mut trans,mut player,mut animation) in query.iter_mut() {
        if left && up {
            trans.translation.x -= speed*0.707;
            trans.translation.y += speed*0.707;
            player.update_state(PlayerState::Walking(Dir::W),&mut animation);
        } else if left && down {
            trans.translation.x -= speed*0.707;
            trans.translation.y -= speed*0.707;
            player.update_state(PlayerState::Walking(Dir::W),&mut animation);
        } else if right && up {
            trans.translation.x += speed*0.707;
            trans.translation.y += speed*0.707;
            player.update_state(PlayerState::Walking(Dir::E),&mut animation);
        } else if right && down {
            trans.translation.x += speed*0.707;
            trans.translation.y -= speed*0.707;
            player.update_state(PlayerState::Walking(Dir::E),&mut animation);
        } else if left {
            trans.translation.x -= speed;
            player.update_state(PlayerState::Walking(Dir::W),&mut animation);
        } else if right {
            trans.translation.x += speed;
            player.update_state(PlayerState::Walking(Dir::E),&mut animation);
        } else if up {
            trans.translation.y += speed;
            player.update_state(PlayerState::Walking(Dir::N),&mut animation);
        } else if down {
            trans.translation.y -= speed;
            player.update_state(PlayerState::Walking(Dir::S),&mut animation);
        } else {
            let dir = {
                match player.state {
                    PlayerState::Walking(dir) => dir,
                    PlayerState::Standing(dir) => dir,
                }
            };
            player.update_state(PlayerState::Standing(dir),&mut animation);
        }
    }
}


