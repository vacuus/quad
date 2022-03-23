use bevy::prelude::*;

#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct KeyboardInputSystem;

#[repr(u8)]
pub enum KeyAction {
    LeftPressed = 0,
    RightPressed = 1,
    DownPressed = 2,
    ClockwiseJustPressed = 3,
    CounterclockwiseJustPressed = 4,
    HardDropJustPressed = 5,
}

pub struct KeyActions {
    bitflags: u8,
}

impl KeyActions {
    pub fn new() -> Self {
        Self { bitflags: 0 }
    }

    fn set_action(&mut self, key_action: KeyAction) {
        self.bitflags |= 1 << key_action as u8;
    }

    pub fn get_action(&self, key_action: KeyAction) -> bool {
        let set = (self.bitflags >> key_action as u8) & 1;
        set != 0
    }

    fn reset_all(&mut self) {
        self.bitflags = 0;
    }
}


pub fn keyboard_input(
    key_events: Res<Input<KeyCode>>,
    mut key_actions: ResMut<KeyActions>,
) {
    use KeyCode::{I, J, K, L, X, Z, Up, Left, Right, Down};
    use self::KeyAction::*;

    key_actions.reset_all();

    if key_events.any_pressed([J, Left]) {
        key_actions.set_action(LeftPressed);
    }

    if key_events.any_pressed([L, Right]) {
        key_actions.set_action(RightPressed);
    }

    if key_events.any_pressed([K, Down]) {
        key_actions.set_action(DownPressed);
    }

    if key_events.just_pressed(X) {
        key_actions.set_action(ClockwiseJustPressed);
    }

    if key_events.just_pressed(Z) {
        key_actions.set_action(CounterclockwiseJustPressed);
    }

    if key_events.any_just_pressed([I, Up]) {
        key_actions.set_action(HardDropJustPressed);
    }
}
