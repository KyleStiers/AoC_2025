use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn get_char_at(char_matrix: &Vec<String>, coordinate: (usize, usize)) -> char {
    let (row, col) = coordinate;
    char_matrix[row].chars().nth(col).unwrap()
}

fn main() -> io::Result<()> {
    // Specify the path to your text file
    let file_path = Path::new("input.txt");
    
    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Initialize matrix with same dims as input
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let rows = lines.len() as usize;
    let cols = lines[0].len() as usize;
    let mut start_coordinate: (usize, usize) = (0, 0);

    // HashMap to track position -> number of paths reaching that position
    let mut curr_paths: HashMap<(usize, usize), usize> = HashMap::new();

    // Find and set start coordinate
    for (row_index, row) in lines.iter().enumerate() {
        for (col_index, c) in row.chars().enumerate() {
            if c == 'S' {
                start_coordinate = (row_index, col_index);
                // Start with 1 path at the position below 'S'
                curr_paths.insert((row_index + 1, col_index), 1);
                break;
            }
        }
        if start_coordinate != (0, 0) {
            break;
        }
    }
    
    println!("Rows: {}, Cols: {}", rows, cols);
    println!("Start coordinate: {:?}", start_coordinate);
    println!("Initial paths: {:?}", curr_paths);

    // Process each row until we reach the last row
    loop {
        // Check if all current positions are on the last row
        let all_on_last_row = curr_paths.keys().all(|(row, _)| *row == rows - 1);
        if all_on_last_row {
            break;
        }

        let mut next_paths: HashMap<(usize, usize), usize> = HashMap::new();

        // Process each current position with its path count
        for ((row, col), path_count) in curr_paths.iter() {
            println!("Processing position ({}, {}) with {} paths", row, col, path_count);
            
            // If already on last row, keep it in next iteration
            if *row == rows - 1 {
                *next_paths.entry((*row, *col)).or_insert(0) += path_count;
                continue;
            }

            // Edge cases: if on left or right edge, move straight down
            if *col == 0 || *col == cols - 1 {
                let next_pos = (*row + 1, *col);
                *next_paths.entry(next_pos).or_insert(0) += path_count;
                println!("  Edge position, moving straight down to {:?}", next_pos);
            }
            // Check if next position has a splitter '^'
            else if get_char_at(&lines, (*row + 1, *col)) == '^' {
                let left_pos = (*row + 1, *col - 1);
                let right_pos = (*row + 1, *col + 1);
                *next_paths.entry(left_pos).or_insert(0) += path_count;
                *next_paths.entry(right_pos).or_insert(0) += path_count;
                println!("  Split at '^', {} paths go left to {:?} and {} paths go right to {:?}", 
                         path_count, left_pos, path_count, right_pos);
            }
            // Otherwise, move straight down
            else {
                let next_pos = (*row + 1, *col);
                *next_paths.entry(next_pos).or_insert(0) += path_count;
                println!("  Moving straight down to {:?}", next_pos);
            }
        }

        curr_paths = next_paths;
        println!("Paths after this iteration: {:?}\n", curr_paths);
    }

    // Calculate total unique paths (sum of all paths in the last row)
    let total_unique_paths: usize = curr_paths.values().sum();
    
    println!("Final positions on last row: {:?}", curr_paths);
    println!("Total unique paths from top to bottom: {}", total_unique_paths);

    Ok(())
}
