use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;

mod menu;
mod game;
mod input;
mod animation;
mod player;
mod movement;
mod resources;
mod hitboxes;
mod components;
mod tile_factory;
mod chunks;

mod constants;

#[derive(Debug,Clone,Hash,Eq,PartialEq)]
pub enum GameState{
    StartMenu,
    Game,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor{
            width: 1200.,
            height: 800.,
            ..Default::default()
        })
        .insert_resource(ClearColor(constants::COLORS[(16*2)+12]))
        .add_state(GameState::Game)
        .add_plugins(DefaultPlugins)
        .add_plugin(
            RonAssetPlugin::<chunks::ChunkMap>::new(&["chunkmap"])
        )
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}
