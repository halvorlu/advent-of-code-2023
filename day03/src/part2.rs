
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
        for j in 0..line.len() {
            if line[j] == '*' {
                sum += gear_ratio(&schematic, i, j);
            }
        }
    }
    println!("{}", sum);
}

fn gear_ratio(schematic: &Vec<Vec<char>>, igear: usize, jgear: usize) -> u32 {
    let mut numbers: Vec<u32> = vec![];
    for i in to_i32(igear)-1..to_i32(igear)+2 {
        if i < 0 || i >= to_i32(schematic.len()) {
            continue;
        }
        let row = &schematic[to_usize(i)];
        let mut numberstart: Option<usize> = None;
        for j in 0..row.len()+1 {
            if j < row.len() && row[j].is_numeric() {
                numberstart = numberstart.or(Some(j));
            } else if numberstart.is_some() {
                // Number has ended
                // Is number next to gear?
                let jnum1 = numberstart.unwrap();
                let jnum2 = j - 1;
                if jnum1 <= jgear + 1 && jnum2 >= jgear - 1 {
                    numbers.push(row[jnum1..jnum2+1].iter().collect::<String>().parse::<u32>().unwrap());
                }
                numberstart = None;
            }
        }
    }
    if numbers.len() == 2 {
        return numbers[0]*numbers[1];
    } else {
        return 0;
    }
}

fn to_i32(i: usize) -> i32 {
    return i32::try_from(i).unwrap();
}

fn to_usize(i: i32) -> usize {
    return usize::try_from(i).unwrap();
}