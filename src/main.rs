fn main() {
    //io::stdout().flush().unwrap();
    //sleep(Duration::from_secs(1));

    println!("{}", generate_grid(20, 20));
}

/// Number of cells must be greater than 0
fn generate_grid(x_cells: usize, y_cells: usize) -> String {
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
    const CELL_INSIDE: &'static str = "   ";

    let mut result: String = String::from("");

    // Generate top row of grid
    result.push(TOP_LEFT);
    for _ in 0..x_cells - 1 {
        result.push_str(HORIZONTAL_LINE);
        result.push(TOP_SPLIT);
    }
    result.push_str(HORIZONTAL_LINE);
    result.push(TOP_RIGHT);

    // Generate middle rows of grid
    for i in 0..(y_cells * 2) - 1 {
        if i % 2 == 0 {
            result.push_str("\n");
            for _ in 0..x_cells {
                result.push(VERTICAL_LINE);
                result.push_str(CELL_INSIDE);
            }
            result.push(VERTICAL_LINE);
        } else {
            result.push_str("\n");
            result.push(LEFT_SPLIT);
            for _ in 0..x_cells - 1 {
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
    for _ in 0..x_cells - 1 {
        result.push_str(HORIZONTAL_LINE);
        result.push(BOTTOM_SPLIT);
    }
    result.push_str(HORIZONTAL_LINE);
    result.push(BOTTOM_RIGHT);

    result
}
