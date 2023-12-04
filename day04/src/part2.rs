
use std::io::{stdin, BufRead};

fn main() {
    let lines: Vec<String> = stdin().lock().lines().map(|x| x.unwrap()).collect();
    let mut counts = vec![1; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        let numbers = line.split(':').nth(1).unwrap();
        let mut parts = numbers.split('|');
        let winning = parts.next().unwrap();
        let mynumbers = parts.next().unwrap().split(' ').filter(|x| x.len() > 0).map(|x| x.parse::<u32>().unwrap());
        let winning: Vec<u32> = winning.split(' ').filter(|x| x.len() > 0).map(|x| x.parse::<u32>().unwrap()).collect();
        let numcount = mynumbers.filter(|x| winning.contains(x)).count();
        let factor = counts[i];
        for j in i+1..i+numcount+1 {
            counts[j] = counts[j] + factor;
        }
    }
    let sum: i32 = counts.iter().sum();
    println!("{}", sum);
}