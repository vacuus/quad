use bevy::prelude::*;
use bevy::input::{ElementState, keyboard::KeyboardInput};


#[repr(u16)]
pub enum KeyAction {
    LeftPressed = 0,
    RightPressed = 1,
    DownPressed = 2,
    ClkwPressed = 3,
    ClkwJustPressed = 4,
    CclwPressed = 5,
    CclwJustPressed = 6,
    HardDropPressed = 7,
    HardDropJustPressed = 8,
}

pub struct KeyActions {
    bitflags: u16,
}

impl KeyActions {
    pub fn new() -> Self {
        Self { bitflags: 0 }
    }

    fn set_action_state(&mut self, key_action: KeyAction, state: ElementState) {
        self.bitflags = match state {
            // set the bit at the appropriate location
            ElementState::Pressed  => self.bitflags |   1 << key_action as u16,
            // reset the bit at the appropriate location
            ElementState::Released => self.bitflags & !(1 << key_action as u16),
        }
    }

    pub fn get_action_state(&self, key_action: KeyAction) -> bool {
        let set = (self.bitflags >> key_action as u16) & 1;
        set != 0
    }
}


pub fn keyboard_input(
    mut key_actions: ResMut<KeyActions>,
    mut key_events: EventReader<KeyboardInput>,
) {
    use self::KeyAction::*;


    let prev_hrddrp_pressed = key_actions.get_action_state(HardDropPressed);
    let prev_clkw_pressed = key_actions.get_action_state(ClkwPressed);
    let prev_cclw_pressed = key_actions.get_action_state(CclwPressed);

    for (state, key_code) in key_events
        .iter()
        .map(|key| (key.state, key.key_code.expect(
            "Key not supported on active keyboard layout (?)",
        )))
    {
        use KeyCode::{W, A, S, D, I, J, K, L, X, Z, Up, Left, Right, Down};

        match key_code {
            W | I | Up => key_actions.set_action_state(HardDropPressed, state),
            A | J | Left  => key_actions.set_action_state(LeftPressed, state),
            S | K | Down  => key_actions.set_action_state(DownPressed, state),
            D | L | Right => key_actions.set_action_state(RightPressed, state),
            Z             => key_actions.set_action_state(CclwPressed, state),
            X             => key_actions.set_action_state(ClkwPressed, state),
            _             => {},
        }
    }

    let mut set_just_pressed = |prev_pressed: bool, p_action, jp_action| {
        let state = if !prev_pressed && key_actions.get_action_state(p_action) {
            ElementState::Pressed
        } else {
            ElementState::Released
        };
        key_actions.set_action_state(jp_action, state);
    };

    set_just_pressed(prev_hrddrp_pressed, HardDropPressed, HardDropJustPressed);
    set_just_pressed(prev_clkw_pressed, ClkwPressed, ClkwJustPressed);
    set_just_pressed(prev_cclw_pressed, CclwPressed, CclwJustPressed);
}
