use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use regex::Regex;

fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty(){
        return Vec::new();
    }
    let transposed = (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect())
        .collect();
    transposed
}

fn main() -> io::Result<()> {
    // Specify the path to your text file
    let file_path = Path::new("test.txt");
    
    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Set up some variables to store things in
    let mut num_matrix = Vec::<Vec<usize>>::new();
    let mut op_matrix = Vec::<Vec<String>>::new();
    let mut storage_vec = Vec::<usize>::new();
    let mut total: usize = 0;

    // Loop over each line in the file
    for line_result in reader.lines() {
        let line = line_result?; // Handle potential errors during line reading
        
        // Set values we'll need
        let digits = Regex::new(r"(\d+)").unwrap();
        let operations = Regex::new(r"([+*])").unwrap();
        let matches: Vec<&str> = digits.find_iter(&line).map(|m| m.as_str()).collect::<Vec<&str>>();
        let ops: Vec<&str> = operations.find_iter(&line).map(|m| m.as_str()).collect::<Vec<&str>>();
        if matches.len() > 0 {
            let row_of_nums: Vec<usize> = matches.iter().map(|s| s.parse::<usize>().unwrap()).collect();
            num_matrix.push(row_of_nums);
            storage_vec = vec![0; num_matrix[0].len()]; // Initialize storage_vec based on first row length
        }
        if ops.len() > 0 {
            let row_of_ops: Vec<String> = ops.iter().map(|s| s.to_string()).collect();
            op_matrix.push(row_of_ops);
        }
    }
    // println!("Matrix as is: {:?}", num_matrix);
    // println!("Operations matrix: {:?}", op_matrix[0]);

    println!("Initialized num_matrix {:?}", num_matrix);
    println!("Transposed matrix test: {:?}", transpose(num_matrix.clone()));
    assert!(num_matrix[0].len() == op_matrix[0].len());

    for (row_index, row) in num_matrix.iter().enumerate() {
        for i in 0..row.len() {
            // For first row just place the value so 0s don't ruin us
            if row_index == 0 {
                storage_vec[i] = row[i];
                continue;
            }
            if op_matrix[0][i] == "+" {
                println!("Adding {} to storage_vec[{}] which is currently {}", row[i], i, storage_vec[i]);
                storage_vec[i] += row[i];
            } else if op_matrix[0][i] == "*" {
                println!("Multiplying {} with storage_vec[{}] which is currently {}", row[i], i, storage_vec[i]);
                storage_vec[i] *= row[i];
            } else {
                panic!("Unknown operation {}", op_matrix[0][i]);
            }
        }
    }


    total = storage_vec.iter().sum();
    println!("Total items in ranges: {}", total);

    Ok(())
}