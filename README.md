# RustConsoleGameEngine
A basic CLI game engine that handles timing, input and rendering to the Windows terminal; heavily inspired by the olcConsoleGameEngine by Javidx9.

The engine is simply a set of modules that help you to create simple games; you have to set up your game loop, assets, etc.

# Usage
## Setup
The library is not currently on crates.io, so to use it, do the following:
- Drag `lib.rs` to your project and either keep it as 'lib' or rename it.
- Add `win32console = "0.1.4"` and `winsafe = "0.0.5"` to your Cargo.toml. (update versions if necessary)

## Game Loop
The following is an example of a set up routine:

```
// SETUP //////////
// parameters in order: width, height, font_width, font_height, title
let mut console = Console::create(80, 30, 16, 16, "Example Game")?;
let mut keyboard = Keyboard::create(vec![Key::ESCAPE, Key::CHAR_Q]);

let mut game_active = true;

while game_active {
    // TIMING //////////
    let delta = time::update_delta();

    // INPUT //////////
    keyboard.update_key_states();

    // GAME LOGIC //////////
    // Key is the Windows VK exported as Key
    if keyboard.get_key_state(Key::ESCAPE).is_pressed() {
        game_active = false;
    }

    if keyboard.get_key_state(Key::CHAR_Z).is_pressed() {
        // do some logic stuff; move pieces around, kill enemy, etc.
    }

    // other logic stuff

    // RENDER //////////
    // renders the screen buffer to the console
    console.update_screen()?;
}
```

Of course, you can structure your code however you like; this isn't a rule, only a starting place.