use bevy::prelude::*;

#[derive(Clone,Default)]
pub struct Frame {
    pub duration:   f32,
    pub sprite: Option<TextureAtlasSprite>,
    pub atlas:  Option<Handle<TextureAtlas>>,
}

#[derive(Component,Clone)]
pub struct Animation {
    pub frames: Vec<Frame>,
    pub timer: Timer,
    pub index: usize,
}

impl Animation {
    pub fn new(frames: Vec<Frame>) -> Self {
        let timer = Timer::from_seconds(0.,false);
        Self {
            frames,
            timer,
            index: 0,
        }
    }
}


pub fn animation_system(
    time: Res<Time>,
    mut query: Query<(&mut Animation, Option<&mut TextureAtlasSprite>, Option<&mut Handle<TextureAtlas>>)>
) {
    for (mut animation,sprite,atlas) in query.iter_mut() {
        
        //increment the timer to see if the frame is over
        animation.timer.tick(time.delta());
        
        //if frame is completed, increment the index to the next frame and update the spret/atlas
        if animation.timer.finished()  {
            //increment index to the next frame
            animation.index = (animation.index + 1) % animation.frames.len();

            //restart the timer to the new frame's duration
            animation.timer = Timer::from_seconds(animation.frames[animation.index].duration,false);

            //set the sprite/atlas to the new frame
            if let Some(new_sprite) = animation.frames[animation.index].sprite.clone() {
                if let Some(mut sprite) = sprite {
                    *sprite = new_sprite;
                } else {
                    //maybe push a new sprite component if there is not one already?
                }
            }

            //set the sprite/atlas to the new frame
            if let Some(new_atlas) = animation.frames[animation.index].atlas.clone() {
                if let Some(mut atlas) = atlas {
                    *atlas = new_atlas;
                } else {
                    //maybe push a new sprite component if there is not one already?
                }
            }
        }
    }
}
