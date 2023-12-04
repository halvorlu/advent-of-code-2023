
use std::io::{stdin, BufRead};

fn main() {
    let lines = stdin().lock().lines().map(|x| x.unwrap());
    let mut sum = 0;
    for line in lines {
        let numbers = line.split(':').nth(1).unwrap();
        let mut parts = numbers.split('|');
        let winning = parts.next().unwrap();
        let mynumbers = parts.next().unwrap().split(' ').filter(|x| x.len() > 0).map(|x| x.parse::<u32>().unwrap());
        let winning: Vec<u32> = winning.split(' ').filter(|x| x.len() > 0).map(|x| x.parse::<u32>().unwrap()).collect();
        let numcount = mynumbers.filter(|x| winning.contains(x)).count();
        if numcount > 0 {
            let base: i32 = 2;
            sum += base.pow(u32::try_from(numcount).unwrap() - 1);
        }
    }
    println!("{}", sum);
}