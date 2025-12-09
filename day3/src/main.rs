use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn highest_joltage(substring: &str, vacancies:i32) -> (String, usize) {
    let mut idx :usize = 0;
    let mut max :i32 = 0;
    let numbers = substring.chars().collect::<Vec<char>>();
    // println!("Now checking substring: {}", substring);
    for i in 0..numbers.len() {
        let curr_num = numbers[i].to_digit(10).unwrap() as i32;
        if curr_num > max && i <= (substring.len() - vacancies as usize) {
            max = curr_num;
            idx = i as usize;
        }
    }
    return (max.to_string(),idx+1);
}

fn main() -> io::Result<()> {
    // Specify the path to your text file
    let file_path = Path::new("input.txt");

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    //track total joltage
    let mut total :i64 = 0;

    // Loop over each line in the file
    for line_result in reader.lines() {
        let max_batteries_on: i32 = 12;
        let line = line_result?;
        let mut batteries_left :i32 = 12;
        let mut top_vals = vec![];
        let mut last_index: usize = 0;

        if line.len() == 1 {
            total += line.parse::<i32>().unwrap() as i64; //if only one digit it must be the highest
            continue;
        }

        // println!("{}",line);
        for on in 0..max_batteries_on {
            if batteries_left == 0 {
                break
            }
            let mut val_to_add:String = String::new();
            let new_index:usize;
            (val_to_add,new_index) = highest_joltage(&line[last_index as usize..], batteries_left);
            last_index += new_index;
            top_vals.push(val_to_add);
            batteries_left -= 1;
        }

        let concatenated_batteries = top_vals.join("").parse::<i64>().unwrap();
        total += concatenated_batteries;
        println!("Top batteries: {}",concatenated_batteries);
    }
    println!("Total joltage: {}",total);
    Ok(())
}
