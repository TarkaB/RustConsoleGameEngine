// Unfinished Tetris demo based on Javidx9's implementation
// Plenty of comments to help if I come back to it.
// Add `[profile.dev] overflow-checks = false` to root Cargo to avoid annoying errors
use engine::{
    input::{Key, Keyboard},
    render::{self, colour, Console, Pixel},
};
use rand::Rng;
use std::{thread, time::Duration};

const ASSETS: [Pixel; 9] = [
    assets::EMPTY,
    assets::PIECE_1,
    assets::PIECE_2,
    assets::PIECE_3,
    assets::PIECE_4,
    assets::PIECE_5,
    assets::PIECE_6,
    assets::PIECE_7,
    assets::BORDER,
];

const SCREEN_WIDTH: u16 = 80;
const SCREEN_HEIGHT: u16 = 30;

const DRAW_OFFSET_X: usize = 30;
const DRAW_OFFSET_Y: usize = 6;

const BOARD_WIDTH: usize = 12;
const BOARD_HEIGHT: usize = 18;

const ZERO_DEGREES: usize = 0;
const NINETY_DEGREES: usize = 1;
const ONE_EIGHTY_DEGREES: usize = 2;
const TWO_SEVENTY_DEGREES: usize = 3;

const TETROMINOS: [[usize; 16]; 7] = [
    // 0 represents empty space, non-zero represents a block,
    // where the number represents an ASSETS index
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

struct Piece {
    pos_x: usize,
    pos_y: usize,
    rotation: usize,
    piece_type: [usize; 16],
}

impl Piece {
    fn new() -> Piece {
        let rand_piece = rand::thread_rng().gen_range(0..6);

        Piece {
            pos_x: BOARD_WIDTH / 2,
            pos_y: 0,
            rotation: 0,
            piece_type: TETROMINOS[rand_piece],
        }
    }

    fn set_position(&mut self, x: usize, y: usize, board: &[usize; BOARD_WIDTH * BOARD_HEIGHT]) {
        if self.does_fit(x, y, self.rotation, board) {
            self.pos_x = x;
            self.pos_y = y;
        }
    }

    fn set_rotation(&mut self, rotation: usize, board: &[usize; BOARD_WIDTH * BOARD_HEIGHT]) {
        if self.does_fit(self.pos_x, self.pos_y, rotation, &board) {
            self.rotation += 1;
        }
    }

    fn does_fit(
        &self,
        x: usize,
        y: usize,
        rotation: usize,
        board: &[usize; BOARD_WIDTH * BOARD_HEIGHT],
    ) -> bool {
        for px in 0..4 {
            for py in 0..4 {
                let index = to_4x4_rotated_index(px, py, rotation);

                // Check within bounds
                if (px + x < BOARD_WIDTH) && (py + y < BOARD_HEIGHT) {
                    let new_index = to_2d_index(px + x, py + y, BOARD_WIDTH);

                    // If the current piece index is a block, and the board index on the
                    // index we want to move that index to is a block, then there's a collision.
                    if self.piece_type[index] != 0 && board[new_index] != 0 {
                        return false;
                    }
                }
            }
        }

        // If no collisions are detected above, then it's all clear.
        true
    }
}

fn to_4x4_rotated_index(x: usize, y: usize, rotation: usize) -> usize {
    // https://www.youtube.com/watch?v=8OK8_tHeCIA
    let index = match rotation % 4 {
        ZERO_DEGREES => y * 4 + x,
        NINETY_DEGREES => 12 + y - (x * 4),
        ONE_EIGHTY_DEGREES => 15 - (y * 4) - x,
        TWO_SEVENTY_DEGREES => 3 - y + (x * 4),
        _ => 0,
    };

    index
}

fn to_2d_index(x: usize, y: usize, array_width: usize) -> usize {
    y * array_width + x
}

pub fn run() {
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
    let mut board = [0; BOARD_WIDTH * BOARD_HEIGHT];

    // Create field borders
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            let index = to_2d_index(x, y, BOARD_WIDTH);

            // 0 = left border, BOARD_WIDTH - 1 = right border, and BOARD_HEIGHT - 1 = bottom border
            if x == 0 || x == BOARD_WIDTH - 1 || y == BOARD_HEIGHT - 1 {
                board[index] = 8; // 8 is the number for the border Pixel in the ASSETS array
            }
        }
    }

    let drop_interval = 20;
    let mut counter = 0;

    let mut piece = Piece::new();

    loop {
        // TICK //////////
        thread::sleep(Duration::from_millis(45));
        counter += 1;

        // INPUT //////////
        keyboard.update_key_states();

        let left = keyboard.get_key_state(Key::LEFT);
        let right = keyboard.get_key_state(Key::RIGHT);
        let down = keyboard.get_key_state(Key::DOWN);
        let key_z = keyboard.get_key_state(Key::CHAR_Z);

        if keyboard.get_key_state(Key::ESCAPE).is_pressed() {
            std::process::exit(0);
        }

        // GAME LOGIC //////////
        if left.is_pressed_or_held() {
            piece.set_position(piece.pos_x - 1, piece.pos_y, &board);
        }
        if right.is_pressed_or_held() {
            piece.set_position(piece.pos_x + 1, piece.pos_y, &board);
        }
        if down.is_pressed_or_held() {
            piece.set_position(piece.pos_x, piece.pos_y + 1, &board);
        }
        if key_z.is_pressed() {
            piece.set_rotation(piece.rotation + 1, &board);
        }

        // Drop the current piece
        if counter == drop_interval {
            counter = 0;

            if !piece.does_fit(piece.pos_x, piece.pos_y + 1, piece.rotation, &board) {
                // Lock the piece
                for x in 0..4 {
                    for y in 0..4 {
                        let board_index =
                            to_2d_index(x + piece.pos_x, y + piece.pos_y, BOARD_WIDTH);
                        let piece_index = to_4x4_rotated_index(x, y, piece.rotation);

                        if piece.piece_type[piece_index] != 0 {
                            board[board_index] = piece.piece_type[piece_index];
                        }
                    }
                }

                // Generate new piece
                piece = Piece::new();

                // Game over
                if !piece.does_fit(piece.pos_x, piece.pos_y, piece.rotation, &board) {
                    std::process::exit(0);
                }
            } else {
                piece.set_position(piece.pos_x, piece.pos_y + 1, &board);
            }
        }

        // RENDER //////////
        // Draw board
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                let index = to_2d_index(x, y, BOARD_WIDTH);
                let board_value = board[index];

                // Each number on the board array represents an index into the
                // ASSETS array; so if the board indexes value is 8, then we
                // draw a BORDER Pixel
                console.draw_pixel(x + DRAW_OFFSET_X, y + DRAW_OFFSET_Y, &ASSETS[board_value]);
            }
        }

        // Draw current piece
        for x in 0..4 {
            for y in 0..4 {
                let index = to_4x4_rotated_index(x, y, piece.rotation);
                let piece_index_value = piece.piece_type[index];

                let (draw_x, draw_y) = (
                    x + piece.pos_x + DRAW_OFFSET_X,
                    y + piece.pos_y + DRAW_OFFSET_Y,
                );

                if piece_index_value != 0 {
                    console.draw_pixel(draw_x, draw_y, &ASSETS[piece_index_value]);
                }
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
        char_value: 'O',
        attributes: colour::FG_DARK_BLUE,
    };
    pub const PIECE_1: Pixel = Pixel {
        char_value: 'A',
        attributes: colour::FG_DARK_RED,
    };
    pub const PIECE_2: Pixel = Pixel {
        char_value: 'B',
        attributes: colour::FG_DARK_GREEN,
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
        attributes: colour::FG_DARK_MAGENTA,
    };
    pub const PIECE_6: Pixel = Pixel {
        char_value: 'F',
        attributes: colour::FG_YELLOW,
    };
    pub const PIECE_7: Pixel = Pixel {
        char_value: 'G',
        attributes: colour::FG_DARK_YELLOW,
    };
}
