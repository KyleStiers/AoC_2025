use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    // Specify the path to your text file
    let file_path = Path::new("test.txt");

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    //track total joltage
    let mut total :i64 = 0;

    // Loop over each line in the file
    for line_result in reader.lines() {
        let line = line_result?;
        let max_on :i32 = 2;
        let values_as_char = vec!['0','1','2','3','4','5','6','7','8','9'];
        let mut top_joltage :i32 = 0;
        let mut top_vals: Vec<i32> = Vec::new();

        if line.len() == 1 {
            total += line.parse::<i32>().unwrap() as i64; //if only one digit it must be the highest
            continue;
        }

        println!("{}",line);
        let mut last_highest_index = 0;
        for val in values_as_char.into_iter().rev() {
            let idx = line.find(val);
            match idx {
                Some(i) => last_highest_index = idx.expect("REASON"),
                None => println!("{} not in this bank",val),
            }
        }
    }

    Ok(())
}
