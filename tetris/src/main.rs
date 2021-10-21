use engine::{
    input::{Key, Keyboard},
    render::{colour, Console, Pixel},
};

const SCREEN_WIDTH: u16 = 80;
const SCREEN_HEIGHT: u16 = 30;

fn main() {
    let mut console = Console::create(SCREEN_WIDTH, SCREEN_HEIGHT, 16, 16, "TETRIS")
        .expect("Could not create Console");
    let mut keyboard =
        Keyboard::create(vec![Key::UP, Key::DOWN, Key::LEFT, Key::RIGHT, Key::CHAR_Z]);

    let mut game_active = true;

    while game_active {
        // TIMING //////////

        // INPUT //////////
        keyboard.update_key_states();

        // GAME LOGIC //////////

        // RENDER //////////
        console
            .update_screen()
            .expect("Could not update the screen");
    }
}
