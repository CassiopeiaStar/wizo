use bevy::prelude::*;

mod menu;
mod game;
mod input;
mod animation;
//mod animations;
mod player;
mod movement;
mod resources;
mod hitboxes;
mod components;

mod constants;

#[derive(Debug,Clone,Hash,Eq,PartialEq)]
pub enum GameState{
    StartMenu,
    Game,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor{
            ..Default::default()
        })
        .insert_resource(ClearColor(constants::COLORS[(16*2)+12]))
        .add_state(GameState::Game)
        .add_plugins(DefaultPlugins)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}
