use std::{io::{stdin, BufRead}, cmp::max};

fn main() {
    let lines = stdin().lock().lines();
    let mut sum = 0;
    for lineresult in lines {
        let line = lineresult.unwrap();
        let draws = line.split(":").nth(1).unwrap().split(";");
        let mut minblue = 0;
        let mut mingreen = 0;
        let mut minred = 0;
        for draw in draws {
            for colorcount in draw.split(",") {
                let mut parts = colorcount.trim().split(" ");
                let count: u32 = parts.next().unwrap().parse().unwrap();
                let color = parts.next().unwrap();
                if color == "blue" {
                    minblue = max(count, minblue);
                } else if color == "green" {
                    mingreen = max(count, mingreen);
                } else if color == "red" {
                    minred = max(count, minred);
                }
            }
        }
        sum += minblue*mingreen*minred;
    }
    println!("{}", sum);
}
