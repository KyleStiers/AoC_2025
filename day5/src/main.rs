use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use regex::Regex;
use rust_lapper::{Interval, Lapper};

type Iv = Interval<usize,usize>;

fn main() -> io::Result<()> {
    // Specify the path to your text file
    let file_path = Path::new("input.txt");
    
    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Set up some variables to store things in
    let mut int_ranges: Vec<(usize,usize)> = Vec::<(usize,usize)>::new();
    let mut intervals: Vec<Iv> = Vec::<Iv>::new();
    let mut avail_items: Vec<usize> = Vec::<usize>::new();
    let mut total_available: usize = 0;


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
                // ranges.push(range.to_string());
                let parts: Vec<usize> = range.split('-').map(|s| s.parse::<usize>().unwrap()).collect();
                assert!(parts.len() == 2);
                let found_interval = Iv{start:parts[0], stop:parts[1]+1, val:0};
                
                intervals.push(found_interval);
                int_ranges.push((parts[0], parts[1]));
            }
        }
        if avail_matches.len() > 0 {
            for item in avail_matches {
                avail_items.push(item.parse::<usize>().unwrap());
            }
        }
    }

    // Example on how to use lapper from the docs
    // let data = vec![Iv{start: 5, stop: 12, val: 0}, Iv{start: 7, stop: 20, val: 0}];
    let mut lapper = Lapper::new(intervals);
    lapper.merge_overlaps(); //man this method is awesome

    // Now check if the items are fresh or spoiled
    for item in avail_items {
        let result = lapper.find(item, item).collect::<Vec<_>>();
        if result.len() > 0 {
            // println!("Item {} found in ranges: {:?}", item, result);
            total_available += 1;
        }
    }

    println!("Total fresh items: {}", total_available);
    let other_total: usize = lapper.iter().map(|iv| iv.stop - iv.start).sum();
    println!("Total items in ranges: {}", other_total);

    Ok(())
}