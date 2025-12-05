use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use regex::Regex;

fn find_new_pos(current_pos:i32) -> i32 {

}

fn main() -> io::Result<()> {
    // Specify the path to your text file
    let file_path = Path::new("input.txt");

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let pos : i32 = 50;

    // Loop over each line in the file
    for line_result in reader.lines() {
        let line = line_result?; // Handle potential errors during line reading
        
        // Set values we'll need
        let re = Regex::new(r"(^[L,R])(\d+)").unwrap();
        if let Some(captures) = re.captures(&line) {
            let direction = captures.get(1).map(|m| m.as_str()).unwrap_or("UP");
            let ticks = captures.get(2).map(|m| m.as_str()).unwrap_or("-1");
            println!("Direction: {}, Ticks: {}", direction, ticks);
        } else {
            println!("We ain't found shit");
        }
    }

    Ok(())
}
