use crossterm_cursor::{self, TerminalCursor};
use std::time::{Duration, SystemTime};

fn main() {
    let cursor = TerminalCursor::new();
    TerminalCursor::hide(&cursor).expect("error while trying to hide cursor");

    let grid = Grid {
        length: 3,
        height: 3,
        alive_cells: vec![(0, 0), (0, 2), (1, 1)],
    };

    let start = SystemTime::now();

    println!("{}", grid.generate_grid());

    let end = SystemTime::now();
    let duration = end.duration_since(start).expect("error");

    println!("{duration:?}");

    TerminalCursor::show(&cursor).expect("error while trying to show cursor");
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
                    match self.alive_cells.contains(&(j, i)) {
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
