use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct BlockBox(pub CollisionRect);

#[derive(Component)]
pub struct HitBox(pub CollisionRect);

#[derive(Component)]
pub struct AttackBox(pub CollisionRect);

#[derive(Component)]
pub struct MovementBox(pub CollisionRect);

#[derive(Component,Debug)]
pub struct CollisionRect {
    //origin is bottom left of the collision rect
    //note that the origin for most entities in bevy is center
    pub pos: Vec2,
    pub size: Vec2,
}

impl CollisionRect {
    pub fn centered(center: Vec2,size: Vec2) -> Self {
        Self {
            pos: center-size/2.,
            size,
        }
    }

    pub fn transformed(&self, transform: &Transform) -> CollisionRect {
        let pos = transform.compute_matrix()
            .transform_point3(self.pos.extend(0.0))
            .truncate();

        /*
        let pos2 = Vec2::new(
            transform.translation.x+(self.pos.x*transform.scale.x),
            transform.translation.y+(self.pos.y*transform.scale.y)
        );
        */

        let size = Vec2::new(
            transform.scale.x * self.size.x,
            transform.scale.y * self.size.y
        );

        CollisionRect {
            size,
            pos
        }
    }

    pub fn shifted(&self,vec: Vec2) -> CollisionRect {
        CollisionRect {
            pos: self.pos+vec,
            size: self.size
        }
    }

    pub fn is_collided(&self,other: &CollisionRect) -> bool {
        self.pos.x < other.pos.x + other.size.x &&
        self.pos.y < other.pos.y + other.size.y &&
        self.pos.x + self.size.x > other.pos.x &&
        self.pos.y + self.size.y > other.pos.y
    }
}
