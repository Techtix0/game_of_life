use crossterm_cursor::{self, TerminalCursor};
use game_of_life::clear_console;
use std::io;
use std::time::SystemTime;

fn main() {
    let mut grid: Grid = get_initial_grid();

    // Hide cursor and clear screen
    let cursor = TerminalCursor::new();
    TerminalCursor::hide(&cursor).expect("error while trying to hide cursor");
    clear_console();

    // Print grid
    println!("{}", grid.generate_grid());

    TerminalCursor::show(&cursor).expect("error while trying to show cursor");
}

fn get_initial_grid() -> Grid {
    let mut length_string = String::new();
    println!("How many cells long should the grid be? ");
    io::stdin()
        .read_line(&mut length_string)
        .expect("Failed to read line");
    let grid_length: u32 = length_string.trim().parse::<u32>().expect("Grid length is not a valid number");


    let mut height_string = String::new();
    println!("How many cells high should the grid be?");
    io::stdin()
        .read_line(&mut height_string)
        .expect("Failed to read line");
    let grid_height: u32 = height_string.trim().parse::<u32>().expect("Grid height is not a valid number");

    let grid = Grid {
        length: grid_length,
        height: grid_height,
        alive_cells: vec![],
    };
    grid
}

#[derive(Default)]
struct Grid {
    length: u32,
    height: u32,
    alive_cells: Vec<(u32, u32)>,
}

impl Grid {
    fn generate_grid(&self) -> String {
        const TOP_LEFT: char = '┌';
        const TOP_RIGHT: char = '┐';
        const BOTTOM_LEFT: char = '└';
        const BOTTOM_RIGHT: char = '┘';
        const HORIZONTAL_LINE: &'static str = "───";
        const VERTICAL_LINE: char = '│';
        const TOP_SPLIT: char = '┬';
        const MIDDLE_SPLIT: char = '┼';
        const BOTTOM_SPLIT: char = '┴';
        const LEFT_SPLIT: char = '├';
        const RIGHT_SPLIT: char = '┤';
        const DEAD_CELL: &'static str = "   ";
        const ALIVE_CELL: &'static str = "▐█▌";

        let mut result: String = String::from("");

        // Generate top row of grid
        result.push(TOP_LEFT);
        for _ in 0..self.length - 1 {
            result.push_str(HORIZONTAL_LINE);
            result.push(TOP_SPLIT);
        }
        result.push_str(HORIZONTAL_LINE);
        result.push(TOP_RIGHT);

        // Generate middle rows of grid
        for i in 0..(self.height * 2) - 1 {
            result.push_str("\n");
            if i % 2 == 0 {
                for j in 0..self.length {
                    result.push(VERTICAL_LINE);
                    match self.alive_cells.contains(&(j, i / 2)) {
                        true => {
                            result.push_str(ALIVE_CELL);
                        }

                        false => {
                            result.push_str(DEAD_CELL);
                        }
                    }
                }
                result.push(VERTICAL_LINE);
            } else {
                result.push(LEFT_SPLIT);
                for _ in 0..self.length - 1 {
                    result.push_str(HORIZONTAL_LINE);
                    result.push(MIDDLE_SPLIT);
                }
                result.push_str(HORIZONTAL_LINE);
                result.push(RIGHT_SPLIT);
            }
        }

        // Generate bottom row of grid
        result.push_str("\n");
        result.push(BOTTOM_LEFT);
        for _ in 0..self.length - 1 {
            result.push_str(HORIZONTAL_LINE);
            result.push(BOTTOM_SPLIT);
        }
        result.push_str(HORIZONTAL_LINE);
        result.push(BOTTOM_RIGHT);

        result
    }
}
