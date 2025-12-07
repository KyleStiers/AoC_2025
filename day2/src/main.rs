use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn to_chunks(string: &str, chunk_size: usize) -> Vec<String> {
    string.chars()
        .collect::<Vec<char>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

fn is_valid(num:i64) -> bool{
    let num_str = num.to_string();
    println!("Checking number: {}", num_str);
    if num_str.len() == 2 {
        if num_str.chars().nth(0) == num_str.chars().nth(1) {
            println!("Invalid number found: {}", num_str);
            return false;
        } else {
            return true;
        }
    }

    let length_of_num :u64 = num.to_string().len().try_into().unwrap();
    let mut divs = divisors::get_divisors(length_of_num);
    divs.push(1);
    for div in divs.iter() {
        let chunks = to_chunks(&num_str, *div as usize);
        if divs.len() == 1 {
            println!("Checking chunks of size {}: {:?}", div, chunks);
        }
        // println!("Checking chunks of size {}: {:?}", div, chunks);
        if chunks.iter().all(|s| s == chunks.first().unwrap()) {
            println!("Invalid number found: {}", num_str);
            return false;
        }
    }

    return true;
}

fn sum_range(total: &mut i64, start: i64, end: i64) {
    for i in start..=end {
        if !is_valid(i){
            *total += i;
        } else{
        }
    }
}

fn main() -> io::Result<()> {
    // Specify the path to your text file
    let file_path = Path::new("input.txt");

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut ranges: Vec<&str>;
    let mut total :i64 = 0;

    // Loop over each line in the file
    for line_result in reader.lines() {
        let line = line_result?; // Handle potential errors during line reading
        ranges = line.split(',').collect();
        
        for range in ranges.iter() {
            // println!("{}", range);
            let start = range.split('-').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>()[0];
            let end = range.split('-').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>()[1];
            // println!("Start: {}, End: {}", start, end);
            sum_range(&mut total, start, end);
        }
    }

    println!("The total sum of all ranges is {}", total);
    Ok(())
}