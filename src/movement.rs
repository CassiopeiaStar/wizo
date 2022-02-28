use bevy::prelude::*;

use crate::components::*;
use crate::player::*;
use crate::chunks::ChunkManager;
use crate::constants::CHUNK_SIZE;

    
fn chunk_to_location(chunk:(i32,i32))-> (f32,f32){
    (chunk.0 as f32 * CHUNK_SIZE.0,
    chunk.1 as f32 * CHUNK_SIZE.1)
}

fn location_to_chunk(location:(f32,f32))-> (i32,i32) {
    let shift = |l:f32,i:f32| -> i32 {
        ((l+(i/2.))/i).floor() as i32
    };
    (shift(location.0,CHUNK_SIZE.0),
    shift(location.1,CHUNK_SIZE.1))
}

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
        trans.translation.z = 500.- ((trans.translation.y - height.0)*0.001) - (trans.translation.x*0.00001);
    }
}


pub fn chunk_switching(
    //windows: Res<Windows>,
    player_query: Query<(&Transform,&Height),(With<Player>,Without<Camera>)>,
    mut chunk_manager: ResMut<ChunkManager>,
    //mut camera_query: Query<&mut CameraDestination,With<Camera>>,
) {
    let (player_transform,player_height) = player_query.single().clone();

    chunk_manager.player_chunk = location_to_chunk((player_transform.translation.x,
        player_transform.translation.y-player_height.0));
}

pub fn moving_camera(
    mut camera_query: Query<&mut Transform,With<MoveToActiveChunk>>,
    chunk_manager: Res<ChunkManager>,
) {
    let speed = 3.;
    let mut trans = camera_query.single_mut();

    let dest = chunk_to_location(chunk_manager.active_chunk);

    if trans.translation.x < dest.0 {
        trans.translation.x = dest.0.min(trans.translation.x + speed);
    }
    if trans.translation.x > dest.0 {
        trans.translation.x = dest.0.max(trans.translation.x - speed);
    }
    if trans.translation.y < dest.1 {
        trans.translation.y = dest.1.min(trans.translation.y + speed);
    }
    if trans.translation.y > dest.1 {
        trans.translation.y = dest.1.max(trans.translation.y - speed);
    }
}


#[derive(Component)]
pub struct MoveToActiveChunk;
