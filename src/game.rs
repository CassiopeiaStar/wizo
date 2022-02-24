
use bevy::prelude::*;

use crate::GameState;
use crate::input::*;
use crate::animation::*;
use crate::player::*;
use crate::movement::*;
use crate::resources::*;
use crate::chunks::*;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
struct PreUpdate;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(InputState::default())
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup))
            //.add_stage_before(CoreStage::Update,"my_preupdate",SystemStage::parallel())
            .add_system_set(SystemSet::on_update(GameState::Game)
                .label(PreUpdate)
                .with_system(game_input)
                .with_system(animation_system)
            )
            .add_system_set(SystemSet::on_update(GameState::Game)
                .after(PreUpdate)
                .with_system(move_player)
                .with_system(movement_system)
                .with_system(player_attack_system)
                //.with_system(debug_draw_hitboxes)
                .with_system(height_system)
                .with_system(chunk_switching)
                .with_system(moving_camera)
                .with_system(chunk_manager)
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

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.scale = Vec3::new(0.2,0.2,1.0);
    let camera_ent = cmd.spawn_bundle(camera)
        .insert(CameraDestination(Vec2::ZERO)).id();

    /*
    let boundary_ent = cmd.spawn_bundle(SpriteBundle {
        texture: asset_server.load("sprites/boundary.png"),
        transform: Transform{
            translation:Vec3::new(0.,0.,-1.),
            scale: Vec3::new(5.,5.,1.),
            ..Default::default()
        },
        ..Default::default()
    }).id();


    cmd.entity(camera_ent).push_children(&[boundary_ent]);
    */

    let player_ent = cmd.spawn_bundle(PlayerBundle::new(&animations,Transform{
        translation:Vec3::new(64.,64.,1.),
        ..Default::default()
    })).id();

    //load_chunk(&mut cmd,&atlas_map,(0,0));
    //load_chunk(&mut cmd,&atlas_map,(1,0));
    //load_chunk(&mut cmd,&atlas_map,(0,-1));
    //load_chunk(&mut cmd,&atlas_map,(1,-1));

    let mut game_data = GameData {
        entities: vec![],
    };

    game_data.entities.push(player_ent);

    cmd.insert_resource(game_data);
    cmd.insert_resource(animations);
    cmd.insert_resource(atlas_map);
    cmd.insert_resource(ChunkManager{
        loaded_chunks: vec![],
        active_chunk: (0,0),
        player_chunk: (0,0),
    })
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


