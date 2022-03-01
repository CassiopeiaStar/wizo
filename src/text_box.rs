
use bevy::prelude::*;
use crate::GameState;
use crate::constants::*;

pub struct TextBoxPlugin;
impl Plugin for TextBoxPlugin {
    fn build(&self,app:&mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::TextBox)
                .with_system(setup)
            )
            .add_system_set(SystemSet::on_update(GameState::TextBox)
                .with_system(update)
            )
            .add_system_set(SystemSet::on_exit(GameState::TextBox)
                .with_system(cleanup)
            );
    }

}

pub struct TextBoxStateData {
    pub text: Vec<String>,
    pub entities: Vec<Entity>,
}

fn setup(
    mut cmd: Commands,
    //mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
    asset_server: Res<AssetServer>,
    mut text_box_state_data: ResMut<TextBoxStateData>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut button_inputs: ResMut<Input<GamepadButton>>,
) {
    keyboard_input.clear();
    button_inputs.clear();
    /*
    let panel_texture_handle: Handle<Image> = asset_server.get_handle("sprites/gold-rimmed-box.png");

    let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(8,8,8,8));

    let nine_slice_ent = cmd.spawn_bundle(
        // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
        // of this entity
        NinePatchBundle {
            style: Style {
                position: Rect{
                    left:Val::Percent(5.),
                    bottom:Val::Percent(5.),
                    right:Val::Percent(5.),
                    top:Val::Percent(80.)
                },
                position_type: PositionType::Absolute,
                size: Size::new(Val::Auto, Val::Auto),
                ..Default::default()
            },
            nine_patch_data: NinePatchData {
                nine_patch: nine_patch_handle,
                texture: panel_texture_handle,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(5.,5.,1.),
                ..Default::default()
            },
            ..Default::default()
        },
    ).id();
    */
    let button_entity = cmd
        .spawn_bundle(ButtonBundle {
            style: Style {
                position: Rect{
                    left:Val::Percent(5.),
                    bottom:Val::Percent(5.),
                    right:Val::Percent(5.),
                    top:Val::Percent(80.)
                },
                position_type: PositionType::Absolute,
                size: Size::new(Val::Auto, Val::Auto),

                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: COLORS[(16*3)+6].into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    text_box_state_data.text[0].clone(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: COLORS[(16*7)+6].into(),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .id();

    text_box_state_data.entities.push(button_entity);
}

fn cleanup(
    mut cmd: Commands,
    state_data: Res<TextBoxStateData>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut button_inputs: ResMut<Input<GamepadButton>>,
) {
    keyboard_input.clear();
    button_inputs.clear();

    for ent in state_data.entities.iter() {
        cmd.entity(*ent).despawn_recursive();
    }

    cmd.remove_resource::<TextBoxStateData>();
}

fn update(
    mut state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    button_inputs: Res<Input<GamepadButton>>,
    gamepads: Res<Gamepads>,
) {
    let mut exit_state = false;
    for gamepad in gamepads.iter().cloned() {
        if button_inputs.just_pressed(GamepadButton(gamepad,GamepadButtonType::South)) {
            exit_state = true;
        }

        if button_inputs.just_pressed(GamepadButton(gamepad,GamepadButtonType::East)) {
            exit_state = true;
        }
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        exit_state = true;
    }
    if keyboard_input.just_pressed(KeyCode::E) {
        exit_state = true;
    }

    if exit_state {
        state.pop().ok();
    }
}
