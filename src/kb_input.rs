use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;


#[repr(u16)]
pub enum KeyAction {
    LeftPressed = 0,
    RightPressed = 1,
    SoftDropPressed = 2,
    ClkwPressed = 3,
    ClkwJustPressed = 4,
    CclwPressed = 5,
    CclwJustPressed = 6,
    HardDropPressed = 7,
    HardDropJustPressed = 8,
}

impl KeyAction {
    fn to_bitmask(self) -> u16 {
        0b1 << self as u16
    }
}

pub struct KeyActions {
    bitflags: u16,
}

impl KeyActions {
    pub fn new() -> Self {
        Self { bitflags: 0 }
    }

    fn set_action_state(&mut self, key_action: KeyAction, signalled: bool) {
        if signalled {
            // set the bit at the appropriate location
            self.bitflags |=  key_action.to_bitmask();
        } else {
            // reset the bit at the appropriate location
            self.bitflags &= !key_action.to_bitmask();
        }
    }

    pub fn get_action_state(&self, key_action: KeyAction) -> bool {
        self.bitflags & key_action.to_bitmask() != 0
    }
}


pub fn keyboard_input(
    mut key_actions: ResMut<KeyActions>,
    mut key_events: EventReader<KeyboardInput>,
) {
    use self::KeyAction::*;
    use KeyCode::*;
    use bevy::input::ButtonState;


    // used to determine state of just pressed action later
    let prev_hrddrp_pressed = key_actions.get_action_state(HardDropPressed);
    let prev_clkw_pressed = key_actions.get_action_state(ClkwPressed);
    let prev_cclw_pressed = key_actions.get_action_state(CclwPressed);

    for (state, key_code) in key_events
        .iter()
        .map(|key|
            (key.state, key.key_code.expect("Key not in keyboard map (?)"))
        )
    {
        let action = match key_code {
            W | I | Up    => HardDropPressed,
            A | J | Left  => LeftPressed,
            S | K | Down  => SoftDropPressed,
            D | L | Right => RightPressed,
            Z             => CclwPressed,
            X             => ClkwPressed,
            _             => continue,
        };
        key_actions.set_action_state(action, state == ButtonState::Pressed);
    }

    let mut set_just_pressed = |prev_pressed: bool, p_action, jp_action| {
        let curr_pressed = key_actions.get_action_state(p_action);
        let just_pressed = !prev_pressed && curr_pressed;
        key_actions.set_action_state(jp_action, just_pressed);
    };

    set_just_pressed(prev_hrddrp_pressed, HardDropPressed, HardDropJustPressed);
    set_just_pressed(prev_clkw_pressed, ClkwPressed, ClkwJustPressed);
    set_just_pressed(prev_cclw_pressed, CclwPressed, CclwJustPressed);
}
