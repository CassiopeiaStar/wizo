use bevy::prelude::*;

use crate::components::*;


pub fn movement_system(
    mut velocities: Query<(&mut Velocity,&mut Transform,&MovementBox)>,
    blocks: Query<(&BlockBox,&Transform),Without<Velocity>>,
    time: Res<Time>,
    /*
    mut query_set: QuerySet<(
        QueryState<(Entity,&mut Velocity,&mut Transform)>,
        QueryState<(&Blocks,&CollisionRect,&Transform),(Without<Velocity>)>
    )>
    */
) {
    for (mut vel,mut mob_tran,mob_coll) in velocities.iter_mut() {
        let mut blocked_x = false;
        let mob_g_coll = mob_coll.0.transformed(&mob_tran).shifted(vel.0*Vec2::X);
        for (block_coll,block_tran) in blocks.iter() {
            let block_g_coll = block_coll.0.transformed(&block_tran);
            if mob_g_coll.is_collided(&block_g_coll) {
                blocked_x = true;
            }
        }
        if !blocked_x {
            mob_tran.translation.x += vel.0.x;
        } else {
            vel.0.x = 0.;
        }

        let mut blocked_y = false;
        let mob_g_coll = mob_coll.0.transformed(&mob_tran).shifted(vel.0*Vec2::Y);
        for (block_coll,block_tran) in blocks.iter() {
            let block_g_coll = block_coll.0.transformed(&block_tran);
            if mob_g_coll.is_collided(&block_g_coll) {
                blocked_y = true;
            }
        }
        if !blocked_y {
            mob_tran.translation.y += vel.0.y;
        } else {
            vel.0.y = 0.;
        }
    }
}


