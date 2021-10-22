use engine::{
    input::{Key, Keyboard},
    render::{self, colour, Console, Pixel},
};

const SCREEN_WIDTH: u16 = 80;
const SCREEN_HEIGHT: u16 = 45;

pub fn run() {
    let mut console = Console::create(SCREEN_WIDTH, SCREEN_HEIGHT, 16, 16, "CONWAY'S GAME OF LIFE")
        .expect("Failed to create Console");
    let mut keyboard = Keyboard::create(vec![Key::ESCAPE, Key::CHAR_Q]);

    loop {
        // TICK //////////

        // INPUT //////////
        keyboard.update_key_states();

        // GAME LOGIC //////////
        if keyboard.get_key_state(Key::ESCAPE).is_pressed() {
            break;
        }

        // RENDER //////////
        console.update_screen().expect("Failed to update screen");
    }
}
