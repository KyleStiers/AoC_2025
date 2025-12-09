use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn check_position {
    // remap

    // if row is lower bound
        // sum symbols in [row][col -1][row][col+1],[row +1][col -1..+1]
    // if col is lower bound
        // sum symbols in [row - 1][col..col+1],[row][col+1],[row +1][col..col+1]
    // if row is upper bound
        // sum symbols in [row - 1][col -1..+1],[row][col -1][row][col+1]
    // if col is upper bound
        // sum symbols in [row - 1][col-1..col],[row][col-1],[row +1][col-1..col]
    // inside the grid
        // sum symbols in [row - 1][col -1..+1],[row][col -1][row][col+1],[row +1][col -1..+1]
}


fn main() -> io::Result<()> {
    // Specify the path to your text file
    let file_path = Path::new("test.txt");

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    //track total joltage
    let mut total :i64 = 0;

    // Loop over each line in the file
    for line_result in reader.lines() {
        let line = line_result?;

    }
    Ok(())
}
