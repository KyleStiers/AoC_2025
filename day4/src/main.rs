use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn check_adjacents(row: usize, col: usize, matrix: &Vec<Vec<i32>>) -> i32 {
    let mut sum :i32 = 0;
    // Catch all on outer edges
    if row == 0 {
        // top left corner
        if col == 0 {
            sum += matrix[row][col + 1] + matrix[row+1][col..=col+1].iter().sum::<i32>();
        }
        // top right corner
        else if col == matrix[0].len() -1 {
            sum += matrix[row][col - 1] + matrix[row+1][col -1..=col].iter().sum::<i32>();
        }
        // anywhere else in top row
        else {
            sum += matrix[row][col - 1] + matrix[row][col + 1] + matrix[row+1][col-1..=col+1].iter().sum::<i32>();
        }
        // sum += matrix[row][col - 1] + matrix[row][col + 1] + matrix[row+1][col -1..col+1].iter().sum::<i32>();
    } else if row == matrix.len() -1 {
        //bottom left corner
        if col == 0 {
            sum += matrix[row - 1][col..=col+1].iter().sum::<i32>() + matrix[row][col + 1];
        }
        //bottom right corner
        else if col == matrix[0].len() -1 {
            sum += matrix[row - 1][col -1..=col].iter().sum::<i32>() + matrix[row][col - 1];
        }
        //anywhere else in bottom row
        else {
            sum += matrix[row - 1][col -1..=col+1].iter().sum::<i32>() + matrix[row][col - 1] + matrix[row][col + 1];
        }
    } else if col == 0 {
        // left edge
        sum += matrix[row - 1][col..=col+1].iter().sum::<i32>() + matrix[row][col + 1] + matrix[row + 1][col..=col+1].iter().sum::<i32>();
    } else if col == matrix[0].len() -1 {
        // right edge
        sum += matrix[row - 1][col -1..=col].iter().sum::<i32>() + matrix[row][col - 1] + matrix[row + 1][col -1..=col].iter().sum::<i32>();
    } else {
        // anywhere non-edge, non-corner
        sum += matrix[row-1][col-1..=col+1].iter().sum::<i32>() + matrix[row][col - 1] + matrix[row][col + 1] + matrix[row+1][col -1..=col+1].iter().sum::<i32>();
    }
    return sum;
}

fn main() -> io::Result<()> {
    // Specify the path to your text file
    let file_path = Path::new("input.txt");
    
    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let rows = lines.len();
    let cols = lines[0].len();
    // println!("Total lines: {}, Total rows: {}", rows, cols);

    let mut total_rolls_removed :i32 = 0;
    let mut input_matrix: Vec<Vec<i32>> = vec![vec![0; cols]; rows];
    let mut copy_matrix: Vec<Vec<i32>> = vec![vec![0; cols]; rows];

    loop {

        // Load up the initial matrix
        for (index, line) in lines.iter().enumerate() {
            for i in 0..line.len() {
                let curr_char = line.chars().nth(i).unwrap();
                if curr_char == '@' {
                    input_matrix[index][i] = 1;
                } else {
                    input_matrix[index][i] = 0;
                }
            }
        }

        // println!("Rows: {:?}", input_matrix);
        let mut tmp_vec = Vec::<String>::new();
        for (row_index, row) in input_matrix.iter().enumerate() {
            let mut adjacent_rolls: i32;
            let mut stringified_row = String::new();
            for (col_index,col_val) in row.iter().enumerate() {
                if *col_val == 1 {
                    adjacent_rolls = check_adjacents(row_index, col_index, &input_matrix);
                    if adjacent_rolls < 4 {
                        total_rolls_removed += 1;
                        stringified_row.push('x')
                    } else{
                        stringified_row.push('@')
                    }
                    // println!("Position ({},{}) Total adjacent_rolls: {}", row_index, col_index, adjacent_rolls);
                } else{
                    stringified_row.push('.');
                    adjacent_rolls = check_adjacents(row_index, col_index, &input_matrix);
                    // println!("Position ({},{}) Total adjacent_rolls: {}", row_index, col_index, adjacent_rolls);
                }   
            }
            println!("{}", stringified_row);
            tmp_vec.push(stringified_row);
        }
        if lines == tmp_vec {
            break;
        } else {
        lines = tmp_vec;
        }
    }

    println!("Total accessible positions: {}", total_rolls_removed);
    Ok(())
}