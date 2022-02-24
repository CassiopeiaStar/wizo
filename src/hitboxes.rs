
use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;


#[derive(Component)]
pub struct DebugBox;

pub fn remove_previous_hitboxes(
    mut cmd: Commands,
    boxes: Query<(Entity,&DebugBox)>,
) {
    for (ent,_) in boxes.iter() {
        println!("Removing {:?}",ent);
        cmd.entity(ent).despawn_recursive();
    }
}

pub fn debug_draw_hitboxes(
    mut cmd: Commands,
    mut debug_boxes: Query<(&DebugBox,&mut Sprite)>,
    blocks: Query<(Entity,&BlockBox,Option<&Children>)>,
    hits: Query<(Entity,&HitBox,Option<&Children>)>,
    attacks: Query<(Entity,&AttackBox,Option<&Children>)>,
    movements: Query<(Entity,&MovementBox,Option<&Children>)>,
) {
    let shifted_alpha = |c:Color| {
        let mut color = c;
        color.set_a(0.5);
        color
    };

    let blue = shifted_alpha(COLORS[(16*2)+8]);
    let red = shifted_alpha(COLORS[(16*2)+4]);
    let green = shifted_alpha(COLORS[(16*2)+12]);
    let yellow = shifted_alpha(COLORS[(16*2)+1]);

    for (ent,bounding_box,children) in blocks.iter() {
        let mut debug_child_found = false;
        if let Some(children) = children {
            for child in children.iter() {
                if let Ok( (_,mut sprite) ) = debug_boxes.get_mut(*child) {
                    sprite.custom_size = Some(bounding_box.0.size);
                    debug_child_found = true;
                }
            }
        }

        if !debug_child_found {
            let bounding_box_ent = cmd.spawn_bundle(SpriteBundle{
                sprite: Sprite {
                    color: blue,
                    custom_size: Some(bounding_box.0.size),
                    ..Default::default()
                },
                transform: Transform{
                    translation: bounding_box.0.center().extend(1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(DebugBox)
            .id();
            cmd.entity(ent).push_children(&[bounding_box_ent]);
        }
    }

    for (ent,bounding_box,children) in movements.iter() {
        let mut debug_child_found = false;
        if let Some(children) = children {
            for child in children.iter() {
                if let Ok( (_,mut sprite) ) = debug_boxes.get_mut(*child) {
                    sprite.custom_size = Some(bounding_box.0.size);
                    debug_child_found = true;
                }
            }
        }

        if !debug_child_found {
            let bounding_box_ent = cmd.spawn_bundle(SpriteBundle{
                sprite: Sprite {
                    color: yellow,
                    custom_size: Some(bounding_box.0.size),
                    ..Default::default()
                },
                transform: Transform{
                    translation: bounding_box.0.center().extend(100.2),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(DebugBox)
            .id();
            cmd.entity(ent).push_children(&[bounding_box_ent]);
        }

    }

    for (ent,bounding_box,children) in hits.iter() {
        let mut debug_child_found = false;
        if let Some(children) = children {
            for child in children.iter() {
                if let Ok( (_,mut sprite) ) = debug_boxes.get_mut(*child) {
                    sprite.custom_size = Some(bounding_box.0.size);
                    debug_child_found = true;
                }
            }
        }

        if !debug_child_found {
            let bounding_box_ent = cmd.spawn_bundle(SpriteBundle{
                sprite: Sprite {
                    color: green,
                    custom_size: Some(bounding_box.0.size),
                    ..Default::default()
                },
                transform: Transform{
                    translation: bounding_box.0.center().extend(100.1),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(DebugBox)
            .id();
            cmd.entity(ent).push_children(&[bounding_box_ent]);
        }

    }

    for (ent,bounding_box,children) in attacks.iter() {
        let mut debug_child_found = false;
        if let Some(children) = children {
            for child in children.iter() {
                if let Ok( (_,mut sprite) ) = debug_boxes.get_mut(*child) {
                    sprite.custom_size = Some(bounding_box.0.size);
                    debug_child_found = true;
                }
            }
        }

        if !debug_child_found {
            let bounding_box_ent = cmd.spawn_bundle(SpriteBundle{
                sprite: Sprite {
                    color: red,
                    custom_size: Some(bounding_box.0.size),
                    ..Default::default()
                },
                transform: Transform{
                    translation: bounding_box.0.center().extend(100.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(DebugBox)
            .id();
            cmd.entity(ent).push_children(&[bounding_box_ent]);
        }

    }
}


