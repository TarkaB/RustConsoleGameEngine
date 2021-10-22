use engine::{
    input::{Key, Keyboard},
    render::{self, colour, Console, Pixel},
};
use rand::Rng;
use std::{thread, time::Duration};

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

const SCREEN_WIDTH: u16 = 80;
const SCREEN_HEIGHT: u16 = 30;

const BOARD_WIDTH: usize = 12;
const BOARD_HEIGHT: usize = 18;

const ZERO_DEGREES: usize = 0;
const NINETY_DEGREES: usize = 1;
const ONE_EIGHTY_DEGREES: usize = 2;
const TWO_SEVENTY_DEGREES: usize = 3;

const TETROMINOS: [[usize; 16]; 7] = [
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
                let current_pos_index = to_4x4_rotated_index(px, py, rotation);
                let piece_index_value = self.piece_type[current_pos_index];

                if (px + x < BOARD_WIDTH) && (py + y < BOARD_HEIGHT) {
                    let new_pos_index = to_2d_index(px + x, py + y, BOARD_WIDTH);

                    let board_value = board[new_pos_index];

                    if piece_index_value != 0 && board_value != 0 {
                        return false;
                    }
                }
            }
        }

        true
    }
}

fn to_4x4_rotated_index(x: usize, y: usize, rotation: usize) -> usize {
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
    let mut board = [0; BOARD_WIDTH * BOARD_HEIGHT];

    // Create field borders
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            let index = to_2d_index(x, y, BOARD_WIDTH);

            if x == 0 || x == BOARD_WIDTH - 1 || y == BOARD_HEIGHT - 1 {
                board[index] = 9;
            }
        }
    }

    let mut game_active = true;

    let mut piece = Piece::new();
    let mut drop_interval = 20;
    let mut counter = 0;

    while game_active {
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

                // Increment score / Check for lines

                // Generate new piece
                piece = Piece::new();

                // Game over
                if !piece.does_fit(piece.pos_x, piece.pos_y, piece.rotation, &board) {
                    game_active = false;
                }
            }

            piece.set_position(piece.pos_x, piece.pos_y + 1, &board);
            counter = 0;
        }

        // RENDER //////////
        // Draw board
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                let index = to_2d_index(x, y, BOARD_WIDTH);
                let board_value = board[index];

                console.draw_pixel(x, y, &ASSETS[board_value]);
            }
        }

        // Draw current piece
        for x in 0..4 {
            for y in 0..4 {
                let index = to_4x4_rotated_index(x, y, piece.rotation);
                let piece_index_value = piece.piece_type[index];

                let (draw_x, draw_y) = (x + piece.pos_x, y + piece.pos_y);

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
        char_value: '#',
        attributes: colour::FG_WHITE,
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
    pub const SHADOW: Pixel = Pixel {
        char_value: render::PIXEL_QUARTER,
        attributes: colour::FG_DARK_GREY,
    };
}
