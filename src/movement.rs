use bevy::prelude::*;

use crate::components::*;
use crate::player::*;
use crate::chunks::ChunkManager;


pub fn movement_system(
    mut velocities: Query<(&mut Velocity,&mut Transform,&GlobalTransform,&MovementBox)>,
    blocks: Query<(&BlockBox,&GlobalTransform),Without<Velocity>>,
    time: Res<Time>,
    /*
    mut query_set: QuerySet<(
        QueryState<(Entity,&mut Velocity,&mut Transform)>,
        QueryState<(&Blocks,&CollisionRect,&Transform),(Without<Velocity>)>
    )>
    */
) {
    for (mut vel,mut mob_tran,mob_global_tran,mob_coll) in velocities.iter_mut() {
        let timed_vel = vel.0*time.delta_seconds();
        let mut blocked_x = false;
        let mob_g_coll = mob_coll.0.transformed(&mob_global_tran).shifted(timed_vel*Vec2::X);
        for (block_coll,block_tran) in blocks.iter() {
            let block_g_coll = block_coll.0.transformed(&block_tran);
            if mob_g_coll.is_collided(&block_g_coll) {
                blocked_x = true;
            }
        }
        if !blocked_x {
            mob_tran.translation.x += timed_vel.x;
        } else {
            vel.0.x = 0.;
        }

        let mut blocked_y = false;
        let mob_g_coll = mob_coll.0.transformed(&mob_global_tran).shifted(timed_vel*Vec2::Y);
        for (block_coll,block_tran) in blocks.iter() {
            let block_g_coll = block_coll.0.transformed(&block_tran);
            if mob_g_coll.is_collided(&block_g_coll) {
                blocked_y = true;
            }
        }
        if !blocked_y {
            mob_tran.translation.y += timed_vel.y;
        } else {
            vel.0.y = 0.;
        }
    }
}

pub fn height_system(
    mut query: Query<(&Height,&mut Transform)>
) {
    for (height,mut trans) in query.iter_mut() {
        trans.translation.z = 500.- ((trans.translation.y - height.0)*0.001);
    }
}


pub fn chunk_switching(
    windows: Res<Windows>,
    player_query: Query<(&Transform,&Height),(With<Player>,Without<Camera>)>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut camera_query: Query<(&Transform,&mut CameraDestination),With<Camera>>,
) {
    let (player_transform,player_height) = player_query.single().clone();
    let (camera_transform,mut camera_destination) = camera_query.single_mut();
    let window = windows.get_primary().unwrap();

    let buffer = 80.;
    let arena_size = Vec2::new(window.width()-buffer,window.height()-buffer);

    if camera_destination.0.x - player_transform.translation.x < 
        -arena_size.x/2. * camera_transform.scale.x {
        chunk_manager.shift_player(1,0);
        camera_destination.0.x += arena_size.x*camera_transform.scale.x;
    }

    if camera_destination.0.x - player_transform.translation.x > 
        arena_size.x/2. * camera_transform.scale.x {
        chunk_manager.shift_player(-1,0);
        camera_destination.0.x -= arena_size.x*camera_transform.scale.x;
    }

    if camera_destination.0.y - player_transform.translation.y + player_height.0 < 
        -arena_size.y/2. * camera_transform.scale.y {
        chunk_manager.shift_player(0,1);
        camera_destination.0.y += arena_size.y*camera_transform.scale.y;
    }

    if camera_destination.0.y - player_transform.translation.y + player_height.0 > 
        arena_size.y/2. * camera_transform.scale.y {
        chunk_manager.shift_player(0,-1);
        camera_destination.0.y -= arena_size.y*camera_transform.scale.y;
    }
}

pub fn moving_camera(
    mut camera_query: Query<(&mut Transform,&CameraDestination),With<Camera>>,
) {
    let speed = 3.;
    let (mut trans,dest) = camera_query.single_mut();
    if trans.translation.x < dest.0.x {
        trans.translation.x = dest.0.x.min(trans.translation.x + speed);
    }
    if trans.translation.x > dest.0.x {
        trans.translation.x = dest.0.x.max(trans.translation.x - speed);
    }
    if trans.translation.y < dest.0.y {
        trans.translation.y = dest.0.y.min(trans.translation.y + speed);
    }
    if trans.translation.y > dest.0.y {
        trans.translation.y = dest.0.y.max(trans.translation.y - speed);
    }
}


#[derive(Component)]
pub struct CameraDestination(pub Vec2);
