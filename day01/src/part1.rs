use std::io::{stdin, BufRead};

fn main()  {
    let lines = stdin().lock().lines();
    let mut sum: u32 = 0;
    for lineresult in lines {
        let mut firstnum: u32 = 10;
        let mut lastnum: u32 = 10;
        let line = lineresult.unwrap();
        for c in line.chars() {
            if c.is_numeric() {
                let num = c.to_digit(10).unwrap();
                if firstnum == 10 {
                    firstnum = num;
                }
                lastnum = num;
            }
        }
        let linenum = firstnum * 10 + lastnum;
        sum += linenum;
    }
    println!("{}", sum);
}