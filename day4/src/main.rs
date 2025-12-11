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
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let rows = lines.len();
    let cols = lines[0].len();
    println!("Total lines: {}, Total rows: {}", rows, cols);
    let mut total_accessible :i32 = 0;
    let mut input_matrix: Vec<Vec<i32>> = vec![vec![0; cols]; rows];
    // input_matrix.iter_mut().for_each(|x| if x=='@' { *x = 1} else { *x = 0});
    // println!("Initialized matrix: {:?}", input_matrix);
    // let mut rows :usize = 0;

    for (index, line) in lines.iter().enumerate() {
        for i in 0..line.len() {
            let curr_char = line.chars().nth(i).unwrap();
            if curr_char == '@' {
                input_matrix[index][i] = 1;
                // let is_ready :bool= check_position(index, i, &input_matrix);
                // println!("Is ready: {}", is_ready);
            } else {
                input_matrix[index][i] = 0;
            }
        }
    }
    println!("Rows: {:?}", input_matrix);
    
    for (row_index, row) in input_matrix.iter().enumerate() {
        let mut available: i32;
        let mut stringified_row = String::new();
        for (col_index,col_val) in row.iter().enumerate() {
            if *col_val == 1 {
                available = check_adjacents(row_index, col_index, &input_matrix);
                if available < 4 {
                    total_accessible += 1;
                    stringified_row.push('x')
                } else{
                    stringified_row.push('@')
                }
                // println!("Position ({},{}) Total available: {}", row_index, col_index, available);
            } else{
                stringified_row.push('.');
                available = check_adjacents(row_index, col_index, &input_matrix);
                // println!("Position ({},{}) Total available: {}", row_index, col_index, available);
            }   
        }
        println!("{}", stringified_row);
    }
    
    println!("Total accessible positions: {}", total_accessible);
    Ok(())
}
