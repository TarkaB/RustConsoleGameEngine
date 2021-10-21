use engine::{
    input::{Key, Keyboard},
    render::{colour, Console, Pixel},
};

fn main() {
    let mut console =
        Console::create(80, 30, 16, 16, "Example Game").expect("Could not create Console");
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
