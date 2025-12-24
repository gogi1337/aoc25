use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum_joltage = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let chars_without_last: Vec<char> = line[0..line.len()-1].chars().collect();
        let mut highest_jolts = 0;

        // println!("{:?} {:?}", chars_without_last, chars);

        if line.len() == 1 || line.len() == 2 {
            highest_jolts = line.parse().unwrap();
        } else {
            let mut highest_first = (0, 0);
            // Getting the first number of the jolts
            for (i, c) in chars_without_last.iter().enumerate() {
                // println!("{i} {c}");
                if highest_first.1 < c.to_digit(10).unwrap() {
                    highest_first = (i, c.to_digit(10).unwrap())
                }
            }
            
            let mut highest_second = (0, 0);
            let chars_for_second: Vec<char> = line[highest_first.0+1..line.len()].chars().collect();
            for (i, c) in chars_for_second.iter().enumerate() {
                // println!("{i} {c}");
                if highest_second.1 < c.to_digit(10).unwrap() {
                    highest_second = (i, c.to_digit(10).unwrap())
                }
            }

            highest_jolts = (highest_first.1 * 10) + highest_second.1;

            println!("{}", highest_jolts);
            sum_joltage += highest_jolts;
        }
    }
    
    println!("Sum of joltage: {sum_joltage}");
}
