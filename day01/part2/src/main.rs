use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("input");
    let mut sum: Vec<i32> = Vec::new();
    for var in input.split_terminator("\n") {
        let str = replace(var.to_string());
        let mut first: char = '-';
        let mut second: char = '-';
        for character in str.chars() {
            if character.is_digit(10) {
                if first == '-' {
                    first = character;
                } else {
                    second = character;
                }
            }
        }
        if second == '-' {
            second = first;
        }
        println!("{}{} {}", first, second, str);
        sum.push(format!("{}{}", first, second).parse::<i32>().unwrap());
    }
    println!("{}", sum.iter().sum::<i32>());
}

fn replace(mut line: String) -> String {
    for a in 0..line.len() {
        let end = line.len();
        for b in (a + 1)..(end + 1) {
            match &line[a..b] {
                "one" => {
                    let s1 = &line[..a];
                    let s2 = "1";
                    let s3 = &line[b..];
                    line = [s1, s2, s3].join("");
                    break;
                }
                "two" => {
                    let s1 = &line[..a];
                    let s2 = "2";
                    let s3 = &line[b..];
                    line = [s1, s2, s3].join("");
                    break;
                }
                "three" => {
                    let s1 = &line[..a];
                    let s2 = "3";
                    let s3 = &line[b..];
                    line = [s1, s2, s3].join("");
                    break;
                }
                "four" => {
                    let s1 = &line[..a];
                    let s2 = "4";
                    let s3 = &line[b..];
                    line = [s1, s2, s3].join("");
                    break;
                }
                "five" => {
                    let s1 = &line[..a];
                    let s2 = "5";
                    let s3 = &line[b..];
                    line = [s1, s2, s3].join("");
                    break;
                }
                "six" => {
                    let s1 = &line[..a];
                    let s2 = "6";
                    let s3 = &line[b..];
                    line = [s1, s2, s3].join("");
                    break;
                }
                "seven" => {
                    let s1 = &line[..a];
                    let s2 = "7";
                    let s3 = &line[b..];
                    line = [s1, s2, s3].join("");
                    break;
                }
                "eight" => {
                    let s1 = &line[..a];
                    let s2 = "8";
                    let s3 = &line[b..];
                    line = [s1, s2, s3].join("");
                    break;
                }
                "nine" => {
                    let s1 = &line[..a];
                    let s2 = "9";
                    let s3 = &line[b..];
                    line = [s1, s2, s3].join("");
                    break;
                }
                &_ => continue,
            }
        }
    }
    return line;
}
