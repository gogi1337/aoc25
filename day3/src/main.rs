use std::{fs::File, io::{BufReader, BufRead}};

// part 2
fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum_joltage = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let bytes = line.as_bytes();
        let mut highest_jolts = 0;
        let mut start_index = 0;
        
        // starting from the left, we need to find 12 maximum digits...
        for digit_number in (0..12).rev() {
            let mut max_digit = 0u8;
            let mut max_index = 0;
            for (i, &b) in bytes[start_index..bytes.len() - digit_number].iter().enumerate() {
                if b > max_digit {
                    max_digit = b;
                    max_index = i;
                    if max_digit == b'9' {
                        break;
                    }
                }
            }
            start_index += max_index + 1;
            highest_jolts = highest_jolts * 10 + (max_digit - b'0') as u64;
        }

        sum_joltage += highest_jolts;
    }

    println!("Sum of joltage: {sum_joltage}");
}

// part1
// fn main() {
//     let file = File::open("input.txt").unwrap();
//     let reader = BufReader::new(file);

//     let mut sum_joltage = 0;

//     for line in reader.lines() {
//         let line = line.unwrap();
//         let chars_without_last: Vec<char> = line[0..line.len()-1].chars().collect();
//         let mut highest_jolts = 0;

//         // println!("{:?} {:?}", chars_without_last, chars);

//         if line.len() == 1 || line.len() == 2 {
//             highest_jolts = line.parse().unwrap();
//         } else {
//             let mut highest_first = (0, 0);
//             // Getting the first number of the jolts
//             for (i, c) in chars_without_last.iter().enumerate() {
//                 // println!("{i} {c}");
//                 if highest_first.1 < c.to_digit(10).unwrap() {
//                     highest_first = (i, c.to_digit(10).unwrap())
//                 }
//             }
            
//             let mut highest_second = (0, 0);
//             let chars_for_second: Vec<char> = line[highest_first.0+1..line.len()].chars().collect();
//             for (i, c) in chars_for_second.iter().enumerate() {
//                 // println!("{i} {c}");
//                 if highest_second.1 < c.to_digit(10).unwrap() {
//                     highest_second = (i, c.to_digit(10).unwrap())
//                 }
//             }

//             highest_jolts = (highest_first.1 * 10) + highest_second.1;

//             println!("{}", highest_jolts);
//             sum_joltage += highest_jolts;
//         }
//     }
    
//     println!("Sum of joltage: {sum_joltage}");
// }
