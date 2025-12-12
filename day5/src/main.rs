use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use regex::Regex;
use rust_lapper::{Interval, Lapper};

fn main() -> io::Result<()> {
    // Specify the path to your text file
    let file_path = Path::new("test.txt");
    
    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Set up some variables to store things in
    let mut ranges: Vec<String> =  Vec::<String>::new();
    let mut avail_items: Vec<i32> = Vec::<i32>::new();
    // let mut intervals: Vec<Interval<i32,i32>> = Vec::<Interval<i32,i32>>::new();

    // Loop over each line in the file
    for line_result in reader.lines() {
        let line = line_result?; // Handle potential errors during line reading
        
        // Set values we'll need
        let re_range = Regex::new(r"(^(\d+)-(\d+))").unwrap();
        let re_avail = Regex::new(r"(^(\d+)$)").unwrap();
        let ranges_matched = re_range.find_iter(&line).map(|m| m.as_str()).collect::<Vec<&str>>();
        let avail_matches = re_avail.find_iter(&line).map(|m| m.as_str()).collect::<Vec<&str>>();
        if ranges_matched.len() > 0 {
            for range in ranges_matched {
                ranges.push(range.to_string());
            }
        }
        if avail_matches.len() > 0 {
            for item in avail_matches {
                avail_items.push(item.parse::<i32>().unwrap());
            }
        }
    }
    println!("Found range: {:?}", ranges);
    println!("Found available items: {:?}", avail_items);

    // Playing with intervals
    type Iv = Interval<usize,usize>;
    let data = vec![Iv{start: 5, stop: 12, val: 0}, Iv{start: 7, stop: 20, val: 0}];
    let mut lapper = Lapper::new(data);
    lapper.merge_overlaps();
    let result = lapper.find(11,11).collect::<Vec<_>>();
    println!("Lapper search result: {:?}", result);

    Ok(())
}