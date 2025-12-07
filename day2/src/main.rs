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

fn other_is_valid(num:i64) -> bool {
    let num_str = num.to_string();
    if num_str.len() == 1 {return true;} //single digit numbers are always valid
    // println!("Checking number: {}", num_str);
    let mut flag = false;
    for i in (1..=num_str.len()/2).rev() {
        // println!("Checking chunks of size {} for number {}", i, num_str);
        if num_str.len() % i == 0 {
            let chunks = to_chunks(&num_str, i as usize);
            let first_chunk = &chunks[0];
            if chunks.iter().all(|s| s == first_chunk){
                println!("Invalid number found: {} with chunk size {}", num_str, i);
                return false;
            }
        }
    }
    true
}

fn sum_range(total: &mut i64, start: i64, end: i64) {
    for i in start..=end {
        if !other_is_valid(i){
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
        let line = line_result?;
        ranges = line.split(',').collect();
        
        for range in ranges.iter() {
            let start = range.split('-').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>()[0];
            let end = range.split('-').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>()[1];
            // println!("Start: {}, End: {}", start, end);
            sum_range(&mut total, start, end);
        }
    }

    println!("The total sum of all ranges is {}", total);
    Ok(())
}