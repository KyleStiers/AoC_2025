use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use regex::Regex;

fn increment(count:&mut i32) {
    *count += 1;
}

fn find_new_pos(current_pos:i32, movement:(char,i32), p2_counter:&mut i32) -> i32 {
    let pos = current_pos;
    let (direction,initial_units) = movement;

    let mut units = 0;

    if initial_units.abs() > 99 {
        units = initial_units % 100;
        //let full_turns = initial_units / 100;
        *p2_counter += initial_units / 100;
    } else {
        units = initial_units;
    }

    if direction == 'L' {
        let result = pos - units;
        if result < 0 {
            *p2_counter += 1;
            return 100 - result.abs();
        }
        else {
            return result;
        }
    } else {
        let result = pos + units;
        if result > 100 {
            *p2_counter += 1;
            return result - 100;
        }
        else {
            return result;
        }
    }
}

fn main() -> io::Result<()> {
    // Specify the path to your text file
    let file_path = Path::new("input.txt");
    let mut protocol1_counter : i32 = 0;
    let mut protocol2_counter : i32 = 0;

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut pos : i32 = 50;

    // Loop over each line in the file
    for line_result in reader.lines() {
        let line = line_result?; // Handle potential errors during line reading
        
        // Set values we'll need
        let re = Regex::new(r"(^[L,R])(\d+)").unwrap();
        if let Some(captures) = re.captures(&line) {
            let direction = captures.get(1).map(|m| m.as_str()).unwrap_or("UP");
            let ticks = captures.get(2).map(|m| m.as_str()).unwrap_or("-1");
            //println!("Direction: {}, Ticks: {}", direction, ticks);
            pos = find_new_pos(pos, (direction.chars().next().unwrap(),ticks.parse().expect("Failed to parse")), &mut protocol2_counter);
            //println!("- The dial is rotated {}{} to point at {}", direction, ticks, pos);
            if pos == 0 {
                //protocol1_counter += 1;
                increment(&mut protocol1_counter);
                increment(&mut protocol2_counter);
                //println!("New value: {}i", counter);
            } else {}
        } else {
            println!("We done or ain't found shit");
        }
        //pos = find_new_pos(direction,ticks.parse().expect("Failed to parse"));
    }

    println!("Protocol 1 value: {}\tProtocol 2 value: {}", protocol1_counter, protocol2_counter);
    Ok(())
}
