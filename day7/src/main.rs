use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn get_char_at(char_matrix: Vec<String>, coordinate: (usize, usize)) -> char {
    let (row, col) = coordinate;
    char_matrix[row].chars().nth(col).unwrap()
}

fn main() -> io::Result<()> {
    // Specify the path to your text file
    let file_path = Path::new("input.txt");
    
    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Set up some variables to store things in
    let mut total :usize = 0;

    // Initialize matrix with same dims as input
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let rows = lines.len() as usize;
    let cols = lines[0].len() as usize;
    let mut input_matrix: Vec<Vec<i32>> = vec![vec![0; cols]; rows];
    let mut start_coordinate: (usize, usize) = (0, 0);
    let mut curr_beam_indices: Vec<(usize, usize)> = Vec::new();
    let mut next_beam_indices: Vec<(usize, usize)> = Vec::new();

    // Find and set start coordinate
    for (row_index, row) in lines.iter_mut().enumerate() {
        for (col_index, c) in row.chars().enumerate() {
            if c == 'S' {
                start_coordinate = (row_index, col_index);
                curr_beam_indices.push((row_index+1, col_index));
                break;
            }
        }
        if start_coordinate != (0, 0) {
            break;
        }
    }
    println!("Rows: {}, Cols: {}", rows, cols);
    println!("Testing coordinate system, start symbol ({}) at {:?}", lines[0].chars().nth(7).unwrap(),start_coordinate);
    println!("Testing equality of index coords: {}", vec![(1,1),(0,1),(2,2)].contains(&(0,1)));
    println!("Initial beam indices before starting: {:?}", curr_beam_indices);

    'checker: loop {
        curr_beam_indices.dedup();
        println!("Current beam indices at start of loop: {:?}", curr_beam_indices);
        for beam_coord in curr_beam_indices.clone().iter() {
            println!("Processing beam at {:?}", beam_coord);
            let (row, col) = *beam_coord;
            if row == rows - 1 {
                break 'checker;
            }
            if col == 0 || col == cols - 1 {
                let index = curr_beam_indices.iter().position(|&x| x == *beam_coord).unwrap();
                curr_beam_indices.remove(index);
                curr_beam_indices.push((row+1, col));
                continue;
            }
            if get_char_at(lines.clone(), (row+1, col)) == '^' {
                let index = curr_beam_indices.iter().position(|&x| x == *beam_coord).unwrap();
                curr_beam_indices.remove(index);
                curr_beam_indices.push((row+1, col-1));
                curr_beam_indices.push((row+1, col+1));
                total += 1;
            }
            else {
                let index = curr_beam_indices.iter().position(|&x| x == *beam_coord).unwrap();
                curr_beam_indices.remove(index);
                curr_beam_indices.push((row+1, col));
            }
        }
    }

    println!("Final beam indices: {:?}\nTotal splits: {}", curr_beam_indices, total);

    Ok(())
}