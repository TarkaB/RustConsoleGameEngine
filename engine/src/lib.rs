pub mod time {
    use std::time::Instant;

    static mut TIME_SINCE_LAST_FRAME: f64 = 0.0;

    pub fn update_delta() -> f64 {
        let mut delta: f64;

        let current_time = Instant::now().elapsed().as_secs_f64();

        unsafe {
            delta = current_time - TIME_SINCE_LAST_FRAME;
            TIME_SINCE_LAST_FRAME = current_time;
        }

        if delta.is_sign_negative() {
            delta = -delta;
        }

        if delta == 0.0 {
            // Make sure NaN is never returned in to_fps()
            delta += 0.0000000000000000000001;
        }

        delta
    }

    pub fn to_fps(delta: f64) -> f64 {
        1.0 / delta
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_delta_time() {
            for _ in 0..10 {
                let delta = update_delta();

                let speed = 10.0;

                let units_per_frame = speed * delta;
                let units_per_second = units_per_frame * to_fps(delta);

                assert!(units_per_second >= 9.5 && units_per_second <= 10.5);
            }
        }
    }
}

pub mod input {
    use std::collections::hash_map::HashMap;
    pub use winsafe::co::VK as Key;
    use winsafe::GetAsyncKeyState;

    pub struct Keyboard {
        keys: HashMap<Key, KeyState>,
    }

    impl Keyboard {
        pub fn create(keys_to_update: Vec<Key>) -> Keyboard {
            let mut keys: HashMap<Key, KeyState> = HashMap::new();

            for vk in keys_to_update {
                keys.insert(vk, KeyState::new());
            }

            Keyboard { keys: keys }
        }

        pub fn update_key_states(&mut self) {
            for key in self.keys.iter_mut() {
                let (vk, state) = key;

                let key_state = GetAsyncKeyState(*vk);

                state.update_state(key_state);
            }
        }

        pub fn get_key_state(&self, key: Key) -> &KeyState {
            match self.keys.get(&key) {
                Some(k) => k,
                None => {
                    panic!("Key was not found; did you add this key when creating the Keyboard?")
                }
            }
        }
    }

    pub struct KeyState {
        pressed: bool,
        held: bool,
        released: bool,
    }

    impl KeyState {
        pub fn is_pressed(&self) -> bool {
            self.pressed
        }

        pub fn is_held(&self) -> bool {
            self.held
        }

        pub fn is_released(&self) -> bool {
            self.released
        }

        pub fn is_open(&self) -> bool {
            !self.pressed && !self.held && !self.released
        }

        pub fn is_pressed_or_held(&self) -> bool {
            self.pressed || self.held
        }

        fn new() -> KeyState {
            KeyState {
                pressed: false,
                held: false,
                released: false,
            }
        }

        fn update_state(&mut self, key_pressed: bool) {
            if key_pressed {
                if !self.pressed && !self.held {
                    self.reset();
                    self.pressed = true;
                } else {
                    self.reset();
                    self.held = true;
                }
            } else if !key_pressed && (self.held || self.pressed) {
                self.reset();
                self.released = true;
            } else {
                self.reset();
            }
        }

        fn reset(&mut self) {
            self.pressed = false;
            self.held = false;
            self.released = false;
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_keyboard_struct() {
            let mut keyboard = Keyboard::create(vec![Key::ESCAPE, Key::CHAR_Q]);

            assert_eq!(keyboard.keys.len(), 2);

            assert!(keyboard.get_key_state(Key::ESCAPE).is_open());
            assert!(keyboard.get_key_state(Key::CHAR_Q).is_open());

            keyboard
                .keys
                .get_mut(&Key::ESCAPE)
                .unwrap()
                .update_state(true);

            assert!(keyboard.get_key_state(Key::ESCAPE).is_pressed());
        }

        #[test]
        fn test_input_logic() {
            let mut key_state = KeyState::new();

            key_state.update_state(true);

            assert!(key_state.is_pressed());

            key_state.update_state(true);

            assert!(key_state.is_held());

            key_state.update_state(false);

            assert!(key_state.is_released());

            key_state.update_state(false);

            assert!(!key_state.is_pressed());
            assert!(!key_state.is_held());
            assert!(!key_state.is_released());
        }
    }
}

pub mod render {
    use std::error::Error;
    pub use win32console::structs::char_info::CharInfo as Pixel;
    use win32console::{
        console::WinConsole,
        structs::{coord::Coord, small_rect::SmallRect},
    };

    pub const PIXEL: char = '█';
    pub const PIXEL_THREE_QUARTERS: char = '▓';
    pub const PIXEL_HALF: char = '▒';
    pub const PIXEL_QUARTER: char = '░';

    pub const PIXEL_EMPTY: Pixel = Pixel {
        char_value: ' ',
        attributes: colour::FG_BLACK,
    };

    pub struct Console {
        pub screen_buffer: Vec<Pixel>,
        screen_info: ScreenInfo,
        console: WinConsole,
    }

    impl Console {
        pub fn create(
            width: u16,
            height: u16,
            font_width: u16,
            font_height: u16,
            title: &str,
        ) -> Result<Console, Box<dyn Error>> {
            let (width, height) = (width as i16, height as i16);

            let handle = WinConsole::create_console_screen_buffer()?;
            let console = WinConsole::with_handle(handle);

            WinConsole::set_active_console_screen_buffer(console.get_handle())?;
            WinConsole::set_title(title)?;

            // Set Window Size //////////
            {
                // In order to change the window size, you must set it to the minimum, set the SCREEN size,
                // THEN set the window size to what you want.
                console.set_window_info(
                    true,
                    &SmallRect {
                        left: 0,
                        top: 0,
                        right: 1,
                        bottom: 1,
                    },
                )?;

                console.set_screen_buffer_size(Coord {
                    x: width,
                    y: height,
                })?;

                console.set_window_info(
                    true,
                    &SmallRect {
                        left: 0,
                        top: 0,
                        right: width - 1,
                        bottom: height - 1,
                    },
                )?;
            }

            // Set Font Size //////////
            {
                let mut new_font = console.get_font_ex(false)?;

                new_font.font_size = Coord {
                    x: font_width as i16,
                    y: font_height as i16,
                };

                console.set_font_ex(new_font, false)?;
            }

            // Create Console
            {
                let mut screen_buffer: Vec<Pixel> = Vec::new();

                for _ in 0..width * height {
                    screen_buffer.push(PIXEL_EMPTY);
                }

                let screen_info = ScreenInfo {
                    area: SmallRect {
                        left: 0,
                        top: 0,
                        right: width,
                        bottom: height,
                    },
                    size: Coord {
                        x: width,
                        y: height,
                    },
                };

                Ok(Console {
                    screen_buffer: screen_buffer,
                    console: console,
                    screen_info: screen_info,
                })
            }
        }

        pub fn update_screen(&mut self) -> Result<(), Box<dyn Error>> {
            self.console.write_output(
                &self.screen_buffer,
                self.screen_info.size,
                Coord::ZERO,
                self.screen_info.area,
            )?;

            Ok(())
        }

        pub fn draw_pixel(&mut self, x: usize, y: usize, pixel: &Pixel) {
            let width = self.screen_info.size.x as usize;
            let index = y * width + x;

            self.screen_buffer[index] = *pixel;
        }

        pub fn draw_string(&mut self, x: usize, y: usize, string: &str, colour: u16) {
            let chars = string.as_bytes();

            let mut pixel = PIXEL_EMPTY;

            for (str_x, c) in chars.iter().enumerate() {
                pixel.char_value = *c as char;
                pixel.attributes = colour;

                self.draw_pixel(str_x as usize + x, y, &pixel);
            }
        }

        pub fn fill(&mut self, x: usize, y: usize, pixel: &Pixel) {
            for screen_x in x..self.get_width() {
                for screen_y in y..self.get_height() {
                    self.draw_pixel(screen_x, screen_y, &pixel);
                }
            }
        }

        pub fn get_pixel(&self, x: usize, y: usize) -> Pixel {
            let width = self.screen_info.size.x as usize;
            let index = y * width + x;

            self.screen_buffer[index]
        }

        pub fn get_width(&self) -> usize {
            self.screen_info.size.x as usize
        }

        pub fn get_height(&self) -> usize {
            self.screen_info.size.y as usize
        }
    }

    struct ScreenInfo {
        area: SmallRect,
        size: Coord,
    }

    pub mod colour {
        // Thank you Javidx9
        pub const FG_BLACK: u16 = 0x0000;
        pub const FG_DARK_BLUE: u16 = 0x0001;
        pub const FG_DARK_GREEN: u16 = 0x0002;
        pub const FG_DARK_CYAN: u16 = 0x0003;
        pub const FG_DARK_RED: u16 = 0x0004;
        pub const FG_DARK_MAGENTA: u16 = 0x0005;
        pub const FG_DARK_YELLOW: u16 = 0x0006;
        pub const FG_GREY: u16 = 0x0007;
        pub const FG_DARK_GREY: u16 = 0x0008;
        pub const FG_BLUE: u16 = 0x0009;
        pub const FG_GREEN: u16 = 0x000A;
        pub const FG_CYAN: u16 = 0x000B;
        pub const FG_RED: u16 = 0x000C;
        pub const FG_MAGENTA: u16 = 0x000D;
        pub const FG_YELLOW: u16 = 0x000E;
        pub const FG_WHITE: u16 = 0x000F;
        pub const BG_BLACK: u16 = 0x0000;
        pub const BG_DARK_BLUE: u16 = 0x0010;
        pub const BG_DARK_GREEN: u16 = 0x0020;
        pub const BG_DARK_CYAN: u16 = 0x0030;
        pub const BG_DARK_RED: u16 = 0x0040;
        pub const BG_DARK_MAGENTA: u16 = 0x0050;
        pub const BG_DARK_YELLOW: u16 = 0x0060;
        pub const BG_GREY: u16 = 0x0070;
        pub const BG_DARK_GREY: u16 = 0x0080;
        pub const BG_BLUE: u16 = 0x0090;
        pub const BG_GREEN: u16 = 0x00A0;
        pub const BG_CYAN: u16 = 0x00B0;
        pub const BG_RED: u16 = 0x00C0;
        pub const BG_MAGENTA: u16 = 0x00D0;
        pub const BG_YELLOW: u16 = 0x00E0;
        pub const BG_WHITE: u16 = 0x00F0;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const PIXEL_WHITE: Pixel = Pixel {
            char_value: PIXEL,
            attributes: colour::FG_WHITE,
        };

        #[test]
        fn test_console() {
            let mut console = Console::create(80, 30, 16, 16, "SPRITE TEST").unwrap();

            // Check that the Sprite was created successfully
            for x in 0..console.get_width() {
                for y in 0..console.get_height() {
                    assert_eq!(console.get_pixel(x, y), PIXEL_EMPTY);
                }
            }

            // Check that set_pixel, get_pixel and draw_sprite are working properly
            console.draw_pixel(0, 0, &PIXEL_WHITE);
            console.draw_pixel(0, 1, &PIXEL_WHITE);

            assert_eq!(console.get_pixel(0, 0), PIXEL_WHITE);
            assert_eq!(console.get_pixel(0, 1), PIXEL_WHITE);
        }

        #[test]
        fn test_draw_string() {
            let mut console = Console::create(80, 30, 16, 16, "SPRITE TEST").unwrap();
            console.draw_string(1, 1, "A", colour::FG_WHITE);

            assert_eq!(
                console.get_pixel(1, 1),
                Pixel {
                    char_value: 'A',
                    attributes: colour::FG_WHITE
                }
            );
        }

        #[test]
        fn test_fill() {
            let mut console = Console::create(80, 30, 16, 16, "SPRITE TEST").unwrap();
            console.fill(0, 0, &PIXEL_WHITE);

            for x in 0..console.get_width() {
                for y in 0..console.get_height() {
                    assert_eq!(console.get_pixel(x, y), PIXEL_WHITE);
                }
            }
        }
    }
}
