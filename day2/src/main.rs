use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::HashSet;
// use regex::Regex;

fn unique_chars(s: &str) -> HashSet<char> {
    s.chars().collect()
}

fn is_valid(num:i64) -> bool{
    let mid = num.to_string().len()/2;
    let num_str = num.to_string();
    // handles when all chars are the same
    if unique_chars(&num_str).len() == 1 && num_str.len() & 2 == 0 {
        println!("Invalid number found: {}", num_str);
        return false;
    }
    //TODO continually subdivide and check for duplication
    if num_str[0..mid] == num_str[mid..] {
        println!("Invalid number found: {}", num_str);
        return false;
    }
    return true;
}

fn sum_range(total: &mut i64, start: i64, end: i64) {
    for i in start..end+1 {
        if !is_valid(i){
            *total += i;
        } else{
        }
    }
}

fn main() -> io::Result<()> {
    // Specify the path to your text file
    let file_path = Path::new("test.txt");

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
            println!("Start: {}, End: {}", start, end);
            sum_range(&mut total, start, end);
        }
    }

    println!("The total sum of all ranges is {}", total);
    Ok(())
}