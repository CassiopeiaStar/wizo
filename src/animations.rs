use bevy::prelude::*;

#[derive(Clone)]
pub struct Frame<T> {
    pub duration: f32,
    pub component: T,
}

#[derive(Component,Clone)]
pub struct Animation<T> {
    pub frames: Vec<Frame<T>>,
    pub timer: Timer,
    pub index: usize,
}

impl<T> Animation<T> {
    pub fn new(frames: Vec<Frame<T>>) -> Self {
        let timer = Timer::from_seconds(0.,false);
        Self {
            frames,
            timer,
            index: 0,
        }
    }
}


pub fn animation_system<T>(
    time: Res<Time>,
    mut query: Query<(&mut Animation<T>, &mut T)>
) where T: Component + Clone {
    for (mut animation,mut component) in query.iter_mut() {
        
        //increment the timer to see if the frame is over
        animation.timer.tick(time.delta());
        
        //if frame is completed, increment the index to the next frame and update the spret/atlas
        if animation.timer.finished()  {
            //increment index to the next frame
            animation.index = (animation.index + 1) % animation.frames.len();

            //restart the timer to the new frame's duration
            animation.timer = Timer::from_seconds(animation.frames[animation.index].duration,false);

            //set the sprite/atlas to the new frame
            *component = animation.frames[animation.index].component.clone();
        }
    }
}
