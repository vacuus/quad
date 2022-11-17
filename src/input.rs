use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;


#[repr(u16)]
pub enum Input {
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

impl Input {
    fn to_bitmask(self) -> u16 {
        0b1 << self as u16
    }
}

#[derive(Resource)]
pub struct Inputs {
    bitflags: u16,
}

impl Inputs {
    pub fn new() -> Self {
        Self { bitflags: 0 }
    }

    fn set_action_state(&mut self, input: Input, signalled: bool) {
        if signalled {
            // set the bit at the appropriate location
            self.bitflags |=  input.to_bitmask();
        } else {
            // reset the bit at the appropriate location
            self.bitflags &= !input.to_bitmask();
        }
    }

    pub fn get_action_state(&self, input: Input) -> bool {
        self.bitflags & input.to_bitmask() != 0
    }
}


pub fn input(
    mut inputs: ResMut<Inputs>,
    mut input_events: EventReader<KeyboardInput>,
) {
    use self::Input::*;
    use KeyCode::*;
    use bevy::input::ButtonState;


    // used to determine state of just pressed action later
    let prev_hrddrp_pressed = inputs.get_action_state(HardDropPressed);
    let prev_clkw_pressed = inputs.get_action_state(ClkwPressed);
    let prev_cclw_pressed = inputs.get_action_state(CclwPressed);

    for (state, key_code) in input_events
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
        inputs.set_action_state(action, state == ButtonState::Pressed);
    }

    let mut set_just_pressed = |prev_pressed: bool, p_action, jp_action| {
        let curr_pressed = inputs.get_action_state(p_action);
        let just_pressed = !prev_pressed && curr_pressed;
        inputs.set_action_state(jp_action, just_pressed);
    };

    set_just_pressed(prev_hrddrp_pressed, HardDropPressed, HardDropJustPressed);
    set_just_pressed(prev_clkw_pressed, ClkwPressed, ClkwJustPressed);
    set_just_pressed(prev_cclw_pressed, CclwPressed, CclwJustPressed);
}
