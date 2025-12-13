use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    // Specify the path to your text file
    let file_path = Path::new("input.txt");
    
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

    let mut current_pairs: Vec<usize> = Vec::new();
    let mut running_tally: Vec<usize> = Vec::new();
    let mut full_op : usize = 0;

    //loop through the loaded array of strings
    for i in 0..line_array[0].len() {
        //placeholder var for op so we can catch the op and use it to condition consolidating all the numbers for that set
        let mut op :String = "".to_string();
        //if the col is all spaces reset vars and continue
        if line_array.iter().all(|c| c.chars().nth(i).unwrap() == ' ') {
            current_pairs = Vec::new();
            full_op = 0;
        } else {
            let nonspace = line_array.iter().map(|c| c.chars().nth(i).into_iter().collect::<String>()).collect::<Vec<String>>();
            let num_from_nonspace = nonspace[..nonspace.len()-1].join("").trim().parse::<usize>().unwrap();
            current_pairs.push(num_from_nonspace);
            op = nonspace[nonspace.len()-1].clone().to_string();
            if op == "*" {
                running_tally.push(current_pairs.iter().product());
            } else if op == "+" {
                running_tally.push(current_pairs.iter().sum());
            }
        }
    }
    println!("Running tally: {:?}", running_tally.iter().sum::<usize>());

    Ok(())
}