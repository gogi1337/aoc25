use std::{fs::File, io::{BufReader, Read}};

/*
    firstly check singles 6 6
    
    secondly check douces 64 64
    that checks 1,2 chars with 3,4

    then check triples 111 111
*/
fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let _ = reader.read_to_string(&mut buf);
    let mut sum_invalid_ids: u64 = 0;
    let mut invalid_ids: Vec<String> = vec![];

    for raw in buf.split(',') {
        let rngmin: u64 = raw[0..raw.find('-').unwrap()].parse().unwrap(); 
        let rngmax: u64 = raw[raw.find('-').unwrap()+1..raw.len()].parse().unwrap();
        let mut falses = 0;
        for n in rngmin..rngmax+1 {
            let n = n.to_string();
            let ch: Vec<char> = n.chars().collect();

            // None of the numbers have leading zeroes by puzzle description
            // if ch[0] == '0' {
            //     falses += 1;
            //     sum_invalid_ids += n.parse::<u64>().unwrap();
            // }
    
            // } else if n.len() == 3 {
            //     if ch[0] == ch[1] && ch[1] == ch[2] {
            //         falses += 1;
            //         sum_invalid_ids += n.parse::<u64>().unwrap();
            //         println!("Invalid id by 3 - {n}");
            //     }
            if n.len() % 2 == 0 {    
                let fh: u64 = n[0..n.len()/2].parse().unwrap();
                let sh: u64 = n[n.len()/2..n.len()].parse().unwrap();
                
                if fh == sh {
                    falses += 1;
                    invalid_ids.push(n.clone());
                    sum_invalid_ids += n.parse::<u64>().unwrap();
                    println!("Invalid id by {} - {n}", n.len());
                }
            } else {
                // @todo: part 2
                // let mut pattern = String::new();
                // let mut prev: Option<char> = None;
                // for c in ch.iter() {
                //     if prev == None {
                //         prev = Some(c.clone());
                //         pattern.push(c.clone());

                //     } else {
                //         if *c == prev.unwrap() {
                //             pattern.push(c.clone());
                //         } else {
                //             let rest = &n[0..n.len()-pattern.len()];

                //             let matches = rest.matches(&pattern).count();
                //             println!("{rest} {pattern} {:?}", matches);
                //         }
                //     }
                // }
            }
        }
        println!("{rngmin} {rngmax} - {falses}");
    }
    println!("Sum of invalid ids: {sum_invalid_ids}");
}
