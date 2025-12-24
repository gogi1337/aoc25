use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut dial: i32 = 50;
    let mut nullified = 0;
    
    for line in reader.lines() {
        let line = line.unwrap();
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let rest = chars.as_str();
        let num = rest.parse::<i32>().unwrap();

        for _ in 0..num {
            if dir == 'L' {
                dial -= 1;
            } else {
                dial += 1;
            }
            
            dial = dial.rem_euclid(100);
            
            if dial == 0 {
                nullified += 1;
            }
        }
    }
    
    println!("{dial} {nullified}");
}
