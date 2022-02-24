use bevy::prelude::*;
use crate::components::*;

#[derive(Clone,Default)]
pub struct Frame {
    pub start:   f32,
    pub actions: Vec<AnimAction>,
}

impl Frame {
    pub fn new(start: f32,actions: Vec<AnimAction>) -> Self {
        Self {
            start,
            actions
        }
    }
}

#[derive(Clone)]
pub enum AnimAction {
    UpdateSprite(TextureAtlasSprite),
    UpdateAtlas(Handle<TextureAtlas>),
    UpdateAttackBox(AttackBox),
    UpdateHitBox(HitBox),
    Repeat,
    DespawnRecursive,
}

#[derive(Component,Clone)]
pub struct Animation {
    frames: Vec<Frame>,
    timer: f32,
    previous_time: f32,
    paused: bool,
}

impl Animation {
    pub fn new(frames: Vec<Frame>) -> Self {
        Self {
            frames,
            timer: 0.,
            previous_time: 0.,
            paused: false,
        }
    }
}


pub fn animation_system(
    mut cmd: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Animation, 
        Option<&mut TextureAtlasSprite>, 
        Option<&mut Handle<TextureAtlas>>,
        Option<&mut HitBox>,
        Option<&mut AttackBox>,
    )>
) {
    for (ent,mut animation,mut sprite,mut atlas,mut hitbox,mut attackbox) in query.iter_mut() {
        if !animation.paused {
            //increment the timer to see if the frame is over
            animation.previous_time = animation.timer;
            animation.timer+=time.delta().as_secs_f32();
            
            let mut reset_animation = false;
            let mut remaining_frames_found = false;
            for frame in animation.frames.iter() {
                if frame.start > animation.timer {
                    remaining_frames_found = true;
                }
                if frame.start < animation.timer && frame.start >= animation.previous_time {
                    for action in frame.actions.iter() {
                        match action {
                            AnimAction::UpdateSprite(new_sprite) => {
                                if let Some(sprite) = sprite.as_deref_mut() {
                                    *sprite = new_sprite.clone();
                                } else {
                                    cmd.entity(ent).insert(new_sprite.clone());
                                }
                            }
                            AnimAction::UpdateAtlas(new_atlas) => {
                                if let Some(atlas) = atlas.as_deref_mut() {
                                    *atlas = new_atlas.clone();
                                } else {
                                    cmd.entity(ent).insert(new_atlas.clone());
                                }
                            }
                            AnimAction::UpdateHitBox(new_hitbox) => {
                                if let Some(hitbox) = hitbox.as_deref_mut() {
                                    *hitbox = new_hitbox.clone();
                                } else {
                                    cmd.entity(ent).insert(new_hitbox.clone());
                                }
                            }
                            AnimAction::UpdateAttackBox(new_attackbox) => {
                                if let Some(attackbox) = attackbox.as_deref_mut() {
                                    *attackbox = new_attackbox.clone();
                                } else {
                                    cmd.entity(ent).insert(new_attackbox.clone());
                                }
                            }
                            AnimAction::Repeat => {
                                reset_animation = true;
                            }
                            AnimAction::DespawnRecursive => {
                                //println!("Removing {:?}",ent);
                                cmd.entity(ent).despawn_recursive();
                            }
                        }
                    }
                }
            }

            if reset_animation {
                animation.timer = 0.;
                animation.previous_time = 0.;
            }

            if !remaining_frames_found {
                //animation.paused = true;
            }
        }
    }
}
