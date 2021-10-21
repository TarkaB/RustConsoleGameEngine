use engine::{
    input::{Key, Keyboard},
    render::{self, colour, Console, Pixel},
};

const SCREEN_WIDTH: u16 = 80;
const SCREEN_HEIGHT: u16 = 30;

const BOARD_WIDTH: usize = 12;
const BOARD_HEIGHT: usize = 18;

/*
const TETROMINOS: [[usize; 16]; 7] = [
    // 4x4 array where 0 represents empty space, and any non-zero number is a block,
    // where the number represents a Pixel.
    // Line
    [0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0],
    // T
    [0, 0, 2, 0, 0, 2, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0],
    // Block
    [0, 0, 0, 0, 0, 3, 3, 0, 0, 3, 3, 0, 0, 0, 0, 0],
    // Z
    [0, 0, 4, 0, 0, 4, 4, 0, 0, 4, 0, 0, 0, 0, 0, 0],
    // S
    [0, 5, 0, 0, 0, 5, 5, 0, 0, 0, 5, 0, 0, 0, 0, 0],
    // L
    [0, 6, 0, 0, 0, 6, 0, 0, 0, 6, 6, 0, 0, 0, 0, 0],
    // J
    [0, 0, 7, 0, 0, 0, 7, 0, 0, 7, 7, 0, 0, 0, 0, 0],
];
*/

const ASSETS: [Pixel; 10] = [
    assets::EMPTY,
    assets::PIECE_1,
    assets::PIECE_2,
    assets::PIECE_3,
    assets::PIECE_4,
    assets::PIECE_5,
    assets::PIECE_6,
    assets::PIECE_7,
    assets::SHADOW,
    assets::BORDER,
];

fn to_2d_index(x: usize, y: usize, array_width: usize) -> usize {
    y * array_width + x
}

fn main() {
    // ENGINE SETUP //////////
    let mut console = Console::create(SCREEN_WIDTH, SCREEN_HEIGHT, 16, 16, "TETRIS")
        .expect("Could not create Console");
    let mut keyboard = Keyboard::create(vec![
        Key::ESCAPE,
        Key::UP,
        Key::DOWN,
        Key::LEFT,
        Key::RIGHT,
        Key::CHAR_Z,
    ]);

    // GAME //////////
    // The board works the same as the TETROMINOS, in its number system.
    let mut board = [0; BOARD_WIDTH * BOARD_HEIGHT];

    // Create field borders
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            let index = to_2d_index(x, y, BOARD_WIDTH);

            // 0 = left border, BOARD_WIDTH - 1 is the right border, and BOARD_HEIGHT - 1 is the bottom border
            // and 1 represents the border Pixel
            if x == 0 || x == BOARD_WIDTH - 1 || y == BOARD_HEIGHT - 1 {
                board[index] = 9;
            }
        }
    }

    let mut game_active = true;

    while game_active {
        // TIMING //////////

        // INPUT //////////
        keyboard.update_key_states();

        // GAME LOGIC //////////

        // RENDER //////////
        // Draw board
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                let index = to_2d_index(x, y, BOARD_WIDTH);
                let board_value = board[index];

                // We draw to the screen ASSETS[index] where index is the current board index's value (int);
                // So if we want 1 on the board array to represent a border, then we want to place
                // the border asset at index 1 on the ASSETS array.
                console.draw_pixel(x, y, &ASSETS[board_value]);
            }
        }

        console
            .update_screen()
            .expect("Could not update the screen");
    }
}

mod assets {
    use super::{colour, render, Pixel};

    pub const EMPTY: Pixel = render::PIXEL_EMPTY;
    pub const BORDER: Pixel = Pixel {
        char_value: '#',
        attributes: colour::FG_DARK_GREY,
    };
    pub const PIECE_1: Pixel = Pixel {
        char_value: 'A',
        attributes: colour::FG_RED,
    };
    pub const PIECE_2: Pixel = Pixel {
        char_value: 'B',
        attributes: colour::FG_GREEN,
    };
    pub const PIECE_3: Pixel = Pixel {
        char_value: 'C',
        attributes: colour::FG_BLUE,
    };
    pub const PIECE_4: Pixel = Pixel {
        char_value: 'D',
        attributes: colour::FG_MAGENTA,
    };
    pub const PIECE_5: Pixel = Pixel {
        char_value: 'E',
        attributes: colour::FG_CYAN,
    };
    pub const PIECE_6: Pixel = Pixel {
        char_value: 'F',
        attributes: colour::FG_YELLOW,
    };
    pub const PIECE_7: Pixel = Pixel {
        char_value: 'G',
        attributes: colour::FG_DARK_YELLOW,
    };
    pub const SHADOW: Pixel = Pixel {
        char_value: render::PIXEL_QUARTER,
        attributes: colour::FG_DARK_GREY,
    };
}
