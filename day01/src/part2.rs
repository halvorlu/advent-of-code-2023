use std::io::{stdin, self, BufRead};

fn find_words(words: [&str; 9], line: &str, firstindex: &mut Option<u32>, firstnum: &mut Option<u32>, lastindex: &mut Option<u32>, lastnum: &mut Option<u32>) {
    for (idx, word) in words.iter().enumerate() {
        let wordvalue: u32 = u32::try_from(idx).unwrap() + 1;
        let wordindex: Option<u32> = line.find(word).map(|c| c.try_into().unwrap());
        let lastwordindex: Option<u32> = line.rfind(word).map(|c| c.try_into().unwrap());
        if let Some(bla) = wordindex {
            if firstindex.is_none() || bla < firstindex.unwrap() {
                *firstindex = Some(bla);
                *firstnum = Some(wordvalue);
            }
        }
        if let Some(bla) = lastwordindex {
            if lastindex.is_none() || bla > lastindex.unwrap() {
                *lastindex = Some(bla);
                *lastnum = Some(wordvalue);
            }
        }
    }
}

fn main()  {
    let lines = stdin().lock().lines();
    let numwords = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut sum: u32 = 0;
    for lineresult in lines {
        let mut firstindex: Option<u32> = None;
        let mut lastindex: Option<u32> = None;
        let mut firstnum: Option<u32> = None;
        let mut lastnum: Option<u32> = None;
        let line = lineresult.unwrap();
        find_words(numwords, &line, &mut firstindex, &mut firstnum, &mut lastindex, &mut lastnum);
        find_words(digits, &line, &mut firstindex, &mut firstnum, &mut lastindex, &mut lastnum);
        let linenum = firstnum.unwrap() * 10 + lastnum.unwrap();
        sum += linenum;
    }
    println!("{}", sum);
}