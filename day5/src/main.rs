use std::{fs::File, io::{BufRead, BufReader}};

fn get_overlap(start1: u64, end1: u64, start2: u64, end2: u64) -> Option<(u64, u64)> {
    let overlap_start = start1.max(start2);
    let overlap_end = end1.min(end2);
    
    if overlap_start <= overlap_end {
        Some((overlap_start, overlap_end))
    } else {
        None
    }
}

fn part2() {
    let mut fresh: Vec<u64> = vec![];
    let mut available: u64 = 0;
    let mut ranges: Vec<(u64, u64)> = vec![];

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    for raw in reader.lines() {
        let raw = raw.unwrap();
        if raw.contains('-') {
            let s: u64 = raw[0..raw.find('-').unwrap()].parse().unwrap(); 
            let e: u64 = raw[raw.find('-').unwrap()+1..raw.len()].parse().unwrap();

            
            let mut toadd = e-s+1;
            // let mut allowed = true;

            for (rs, re) in ranges.iter() {
                if let Some(overlap) = get_overlap(*rs, *re, s, e) {
                    let countoverlap = overlap.1 - overlap.0;
                    println!("{toadd} 1");
                    toadd -= countoverlap;
                    println!("{toadd} 2");
                }
            }
            ranges.push((s, e));
            available += toadd;
        }
    }

    println!("Available Ids: {available}");
}

fn part1() {
    let mut to_check: Vec<u64> = vec![];
    let mut fresh: Vec<u64> = vec![];
    let mut fresh_count = 0;
    
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    for raw in reader.lines() {
        let raw = raw.unwrap();
        if let Ok(id) = raw.parse::<u64>() {
            to_check.push(id);
        }
    }

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    for raw in reader.lines() {
        let raw = raw.unwrap();
        if raw.contains('-') {
            let s: u64 = raw[0..raw.find('-').unwrap()].parse().unwrap(); 
            let e: u64 = raw[raw.find('-').unwrap()+1..raw.len()].parse().unwrap();
            
            for id in &to_check {
                if *id >= s && *id <= e && !fresh.contains(&*id) {
                    println!("{id}");
                    fresh.push(id.clone());
                    fresh_count += 1;
                }
            }
            
        }
    }

    println!("Available fresh: {fresh_count}");
}

fn main() {
    part2();
}