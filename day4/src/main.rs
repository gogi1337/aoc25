use std::{fs::File, io::{BufRead, BufReader}};

fn part2(grid: &mut Vec<Vec<char>>, tr: &mut usize) {
    let mut removed: usize = 0;
    println!("{tr}");
    
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '0' { break; }
            if grid[i][j] == '@' {
                let mut neighbours: Vec<char> = vec![];
                let mut l = '0';
                if j != 0 { l = grid[i][j-1] }
                let r = grid[i][j+1];
                neighbours.push(l);
                neighbours.push(r);
                
                let mut t = '0';
                let mut lt = '0';
                let mut rt = '0';
                if i != 0 {
                    t = grid[i-1][j];
                    if j != 0 { lt = grid[i-1][j-1]; }
                    rt = grid[i-1][j+1];
                }
                neighbours.push(t);
                neighbours.push(lt);
                neighbours.push(rt);

                let b = grid[i+1][j];
                let mut lb = '0';
                if j != 0 {
                    lb = grid[i+1][j-1]
                }
                let rb = grid[i+1][j+1];
                neighbours.push(b);
                neighbours.push(lb);
                neighbours.push(rb);                
                
                let count = neighbours.iter().filter(|n| **n == 'x' || **n == '@').count();
                if count < 4 {
                    grid[i][j] = '.';
                    removed += 1;
                    *tr += 1;
                }
            }
            print!("{}", grid[i][j]);
        }
        println!("");
    }

    if removed != 0 { part2(grid, tr); }
}

fn part1() {
    let file = File::open("input.txt").unwrap();
    let lc = BufReader::new(file).lines().count();
    let mut grid: Vec<Vec<char>> = vec![vec!['0'; 150]; lc+1];
    
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for (i, l) in reader.lines().enumerate() {
        for (j, c) in l.unwrap().chars().into_iter().enumerate() {
            // println!("{i} {j} - {c}");
            grid[i][j] = c;
        }
    }

    let mut tr = 0;
    
    part2(&mut grid, &mut tr);
    println!("Removed: {tr}");
}

fn main() {
    part1();
}
