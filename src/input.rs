use bevy::prelude::*;

#[derive(Default)]
pub struct InputState {
    pub accept: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub attack: bool,
    pub chat: bool,
}


pub fn game_input(
    mut input_state: ResMut<InputState>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut left = false;
    let mut right = false;
    let mut up = false;
    let mut down = false;
    let mut accept = false;
    let mut attack = false;
    let mut chat = false;
    for gamepad in gamepads.iter().cloned() {
        if let Some(left_stick_x) = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX)) {
                if left_stick_x < -0.5 {left = true;}
                if left_stick_x >  0.5 {right = true;}
            }
        if let Some(left_stick_y) = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickY)) {
                if left_stick_y < -0.5 {down = true;}
                if left_stick_y >  0.5 {up = true;}
            }
        if button_inputs.pressed(GamepadButton(gamepad,GamepadButtonType::DPadDown)) {
            down = true;
        }
        if button_inputs.pressed(GamepadButton(gamepad,GamepadButtonType::DPadUp)) {
            up = true;
        }
        if button_inputs.pressed(GamepadButton(gamepad,GamepadButtonType::DPadRight)) {
            right = true;
        }
        if button_inputs.pressed(GamepadButton(gamepad,GamepadButtonType::DPadLeft)) {
            left = true;
        }
        if button_inputs.just_pressed(GamepadButton(gamepad,GamepadButtonType::South)) {
            accept = true;
        }
        if button_inputs.just_pressed(GamepadButton(gamepad,GamepadButtonType::West)) {
            attack = true;
        }
    }

    if keyboard_input.pressed(KeyCode::A) {left = true;}
    if keyboard_input.pressed(KeyCode::D) {right = true;}
    if keyboard_input.pressed(KeyCode::W) {up = true;}
    if keyboard_input.pressed(KeyCode::S) {down = true;}
    if keyboard_input.just_pressed(KeyCode::F) {attack = true;}
    if keyboard_input.just_pressed(KeyCode::E) {chat = true;}

    *input_state = InputState {
        left,
        right,
        up,
        down,
        accept,
        attack,
        chat,
    };
}

fn print_input(
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
) {
    for gamepad in gamepads.iter().cloned() {
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::South)) {
            info!("{:?} just pressed South", gamepad);
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::South)) {
            info!("{:?} just released South", gamepad);
        }

        let right_trigger = button_axes
            .get(GamepadButton(gamepad, GamepadButtonType::RightTrigger2))
            .unwrap();
        if right_trigger.abs() > 0.01 {
            info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
        }

        let left_stick_x = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        let left_stick_y = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        
        if left_stick_x.abs() > 0.01 {
            info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
        }
    }
}
