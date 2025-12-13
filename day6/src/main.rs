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
    let mut line_array :Vec<String> = Vec::new();

    // Loop over each line in the file
    for line_result in reader.lines() {
        let line = line_result?; // Handle potential errors during line reading
        // Load strings into vectors
        line_array.push(line.chars().rev().collect());
    }

    for (row_index, line) in line_array.iter().enumerate() {
        println!("Line {}: {}", row_index, line);
    }

    for i in 0..line_array[0].len() {
        let mut current_pairs: Vec<String> = Vec::new();
        let mut op :String = ''
        if line_array.iter().all(|c| c.chars().nth(i).unwrap() == ' ') {
            println!("Column {} is all spaces", i);
        } else {
            let nonspace = line_array.iter().map(|c| c.chars().nth(i).into_iter().collect::<String>()).collect::<Vec<String>>();
            // top_vals.join("").parse::<i64>().unwrap();
            println!("Column {} has chars: {:?}", i, &nonspace[..nonspace.len()-1]);
            let num_from_nonspace = nonspace[..nonspace.len()-1].join("").trim().parse::<usize>().unwrap();
            println!("Parsed number: {}", num_from_nonspace);

        }
    }

    // println!("Loaded lines: {:?}", line_array);

    Ok(())
}