use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("input");
    let mut sum: Vec<i32> = Vec::new();
    for var in input.split_terminator("\n") {
        let mut first: char = '-';
        let mut second: char = '-';
        for character in var.chars() {
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
        sum.push(format!("{}{}", first, second).parse::<i32>().unwrap());
    }
    println!("{}", sum.iter().sum::<i32>());
}
