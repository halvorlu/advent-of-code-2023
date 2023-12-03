
use std::io::{stdin, BufRead};

fn main() {
    let lines = stdin().lock().lines();
    let mut sum = 0;
    let mut schematic: Vec<Vec<char>> = vec![];
    for lineresult in lines {
        schematic.push(lineresult.unwrap().chars().collect());
    }
    for i in 0..schematic.len() {
        let line = &schematic[i];
        let mut numberstart: Option<usize> = None;
        for j in 0..line.len() + 1 {
            if j < line.len() && line[j].is_numeric() {
                numberstart = numberstart.or(Some(j));
            } else if numberstart.is_some() {
                // Number ended previous column
                let numberend = j - 1;
                if find_symbol(&schematic, i, numberstart.unwrap(), numberend) {
                    let numberstring: String = line[numberstart.unwrap()..numberend+1].iter().collect();
                    //println!("{}", numberstring);
                    let number: u32 = numberstring.parse().unwrap();
                    sum += number;
                }
                numberstart = None;
            }
        }
    }
    println!("{}", sum);
}

fn find_symbol(schematic: &Vec<Vec<char>>, inum: usize, jnum1: usize, jnum2: usize) -> bool {
    for i in to_i32(inum)-1..to_i32(inum)+2 {
        if i < 0 || i >= to_i32(schematic.len()) {
            continue;
        }
        let row = &schematic[to_usize(i)];
        for j in to_i32(jnum1)-1..to_i32(jnum2)+2 {
            if j < 0 || j >= to_i32(row.len()) {
                continue;
            }
            if !row[to_usize(j)].is_numeric() && row[to_usize(j)] != '.' {
                return true;
            }
        }
    }
    return false;
}

fn to_i32(i: usize) -> i32 {
    return i32::try_from(i).unwrap();
}

fn to_usize(i: i32) -> usize {
    return usize::try_from(i).unwrap();
}