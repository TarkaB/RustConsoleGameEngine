use engine::{
    input::{Key, Keyboard},
    render::{self, colour, Console, Pixel},
};

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 45;

const ASSETS: [Pixel; 2] = [assets::DEAD_CELL, assets::LIVE_CELL];

const TICK_INTERVAL: u64 = 15; // ms

pub fn run() {
    let mut console = Console::create(
        SCREEN_WIDTH as u16,
        SCREEN_HEIGHT as u16,
        8,
        8,
        "CONWAY'S GAME OF LIFE",
    )
    .expect("Failed to create Console");
    let mut keyboard = Keyboard::create(vec![Key::ESCAPE, Key::CHAR_Q]);

    let mut board = [0; SCREEN_WIDTH * SCREEN_HEIGHT];

    board[2 * SCREEN_WIDTH + 1] = 1;
    board[2 * SCREEN_WIDTH + 2] = 1;
    board[2 * SCREEN_WIDTH + 3] = 1;
    board[1 * SCREEN_WIDTH + 3] = 1;
    board[0 * SCREEN_WIDTH + 2] = 1;

    loop {
        // TICK //////////
        std::thread::sleep(std::time::Duration::from_millis(TICK_INTERVAL));

        // INPUT //////////
        keyboard.update_key_states();

        if keyboard.get_key_state(Key::ESCAPE).is_pressed() {
            break;
        }
        // ALGORITHM //////////
        // Create a new board
        let mut new_board = [0; SCREEN_WIDTH * SCREEN_HEIGHT];

        // Alter board according to algo
        for x in 1..SCREEN_WIDTH - 1 {
            for y in 1..SCREEN_HEIGHT - 1 {
                let current_cell_index = y * SCREEN_WIDTH + x;
                let mut neighbour_cells_indexes = [0; 8];

                let mut live_neighbour_count = 0;

                neighbour_cells_indexes[0] = (y - 1) * SCREEN_WIDTH + (x - 1); // Top Left
                neighbour_cells_indexes[1] = (y - 1) * SCREEN_WIDTH + x; // Top Middle
                neighbour_cells_indexes[2] = (y - 1) * SCREEN_WIDTH + (x + 1); // Top Right
                neighbour_cells_indexes[3] = y * SCREEN_WIDTH + (x - 1); // Middle Left
                neighbour_cells_indexes[4] = y * SCREEN_WIDTH + (x + 1); // Middle Right
                neighbour_cells_indexes[5] = (y + 1) * SCREEN_WIDTH + (x - 1); // Bottom Left
                neighbour_cells_indexes[6] = (y + 1) * SCREEN_WIDTH + x; // Bottom Middle
                neighbour_cells_indexes[7] = (y + 1) * SCREEN_WIDTH + (x + 1); // Bottom Right

                for i in neighbour_cells_indexes {
                    if board[i] == 1 {
                        live_neighbour_count += 1;
                    }
                }

                if board[current_cell_index] == 1 {
                    if live_neighbour_count == 2 || live_neighbour_count == 3 {
                        new_board[current_cell_index] = 1;
                    } else {
                        new_board[current_cell_index] = 0;
                    }
                } else {
                    if live_neighbour_count == 3 {
                        new_board[current_cell_index] = 1;
                    }
                }
            }
        }

        // Set the board to the new board
        board = new_board;

        // RENDER //////////
        // Render board
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let index = y * SCREEN_WIDTH + x;
                let cell_state = board[index];

                console.draw_pixel(x, y, &ASSETS[cell_state]);
            }
        }

        console.update_screen().expect("Failed to update screen");
    }
}

mod assets {
    use super::{colour, render, Pixel};

    pub const LIVE_CELL: Pixel = Pixel {
        char_value: render::PIXEL,
        attributes: colour::FG_WHITE,
    };

    pub const DEAD_CELL: Pixel = Pixel {
        char_value: render::PIXEL,
        attributes: colour::FG_BLACK,
    };
}
