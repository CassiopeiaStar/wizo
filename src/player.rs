
use bevy::prelude::*;

use crate::components::*;
use crate::animation::*;
use crate::resources::*;
use crate::input::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite: TextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub player: Player,
    pub animation: Animation,
    pub velocity: Velocity,
    pub movement_box: MovementBox,
    pub height: Height,
}

impl PlayerBundle {
    pub fn new(animations: &AnimationMap,transform: Transform) -> Self {
        Self {
            sprite: Default::default(),
            texture_atlas: Default::default(),
            transform,
            global_transform: Default::default(),
            visibility: Default::default(),
            animation: animations.0.get(&AnimationName::Standing(Dir::S)).unwrap().clone(),
            velocity:Velocity(Vec2::ZERO),
            player: Player::new(),
            movement_box: MovementBox(CollisionRect::centered(
                Vec2::new(0.,-12.),
                Vec2::new(8.,8.)
            )),
            height: Height(12.),
        }
    }
}

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

pub fn move_player(
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
    let speed = 50.;
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

pub fn player_attack_system(
    mut cmd: Commands,
    mut input_state: ResMut<InputState>,
    player: Query<(Entity,&Player)>,
    animations: Res<AnimationMap>,
){
    let (player_ent,player) = player.single();

    if input_state.attack {
        input_state.attack = false;
        match player.state {
            PlayerState::Standing(dir) | PlayerState::Walking(dir) => {
                let transform = match dir {
                    Dir::N => Transform {
                        translation: Vec3::new(0.,-8.,-0.1),
                        rotation: Quat::from_rotation_z(std::f32::consts::PI/2.),
                        ..Default::default()
                    },
                    Dir::E => Transform {
                        translation: Vec3::new(0.,-8.,0.1),
                        ..Default::default()
                    },
                    Dir::S => Transform {
                        translation: Vec3::new(0.,-8.,0.1),
                        rotation: Quat::from_rotation_z(-std::f32::consts::PI/2.),
                        ..Default::default()
                    },
                    Dir::W => Transform {
                        translation: Vec3::new(0.,-8.,0.1),
                        rotation: Quat::from_rotation_z(std::f32::consts::PI),
                        ..Default::default()
                    }
                };

                let sword_ent = cmd.spawn_bundle(SpriteSheetBundle{
                    transform,
                    ..Default::default()
                })
                .insert(animations.0.get(&AnimationName::Sword).unwrap().clone())
                .id();

                cmd.entity(player_ent).push_children(&[sword_ent]);
            }
        }
    }
}

pub fn sign_reading_system(
    input_state: ResMut<InputState>,
    player: Query<(&Player,&GlobalTransform),Without<Sign>>,
    signs: Query<(&Sign,&GlobalTransform),Without<Player>>
) {
    let distance = 20.;
    if input_state.chat {
        let (_,player_transform) = player.single();
        for (sign,sign_transform) in signs.iter() {
            if (player_transform.translation.x - sign_transform.translation.x).abs() < distance &&
                (player_transform.translation.y - sign_transform.translation.y).abs() < distance {
                    dbg!(sign);
                }

        }
    }
}
