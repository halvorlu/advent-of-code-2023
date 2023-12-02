use std::io::{stdin, BufRead};

fn is_possible(draw: &str) -> bool {
    for colorcount in draw.split(",") {
        let mut parts = colorcount.trim().split(" ");
        let count: u32 = parts.next().unwrap().parse().unwrap();
        let color = parts.next().unwrap();
        if (color == "blue" && count > 14) || (color == "green" && count > 13)
        || (color == "red" && count > 12) {
            return false;
        }
    }
    return true;
}

fn main() {
    let lines = stdin().lock().lines();
    let mut sum = 0;
    for lineresult in lines {
        let line = lineresult.unwrap();
        let mut parts = line.split(":");
        let part1 = parts.next().unwrap();
        let id: u32 = part1.split(" ").nth(1).unwrap().parse().unwrap();
        let draws = parts.next().unwrap().split(";");
        let mut possible = true;
        for draw in draws {
            if !is_possible(draw) {
                possible = false;
                break;
            }
        }
        if possible {
            sum += id;
        }
    }
    println!("{}", sum);
}
