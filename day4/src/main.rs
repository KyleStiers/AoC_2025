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

    }
    Ok(())
}
