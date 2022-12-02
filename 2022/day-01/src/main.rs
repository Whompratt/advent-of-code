use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let calorie_counts = parse_input(String::from("./input.txt"));

    let largest_calorie_count = get_largest_calorie_count(&calorie_counts);
    println!("Largest calorie count: {}", largest_calorie_count);

    let largest_three_calorie_counts = get_three_largest_calorie_counts(&calorie_counts);
    println!(
        "Sum of largets three calorie counts: {}",
        largest_three_calorie_counts.iter().sum::<i32>()
    );
}

fn get_three_largest_calorie_counts(calorie_counts: &Vec<i32>) -> Vec<i32> {
    let mut top_three = Vec::with_capacity(4);
    for v in calorie_counts.iter().copied() {
        top_three.push(v);

        let mut i = top_three.len() - 1;
        while i > 0 {
            if top_three[i] <= top_three[i - 1] {
                break;
            }
            let t = top_three[i];
            top_three[i] = top_three[i - 1];
            top_three[i - 1] = t;

            i -= 1;
        }

        if top_three.len() > 3 {
            top_three.pop();
        }
    }
    top_three
}

fn get_largest_calorie_count(calorie_counts: &Vec<i32>) -> i32 {
    *calorie_counts.iter().max().unwrap()
}

fn parse_input(input: String) -> Vec<i32> {
    let mut index: usize = 0;
    let mut calorie_counts: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(calories) = line {
                if calories == "" {
                    index = index + 1;
                } else {
                    if index + 1 > calorie_counts.len() {
                        calorie_counts.push(0);
                    }
                    calorie_counts[index] =
                        calorie_counts[index] + calories.parse::<i32>().unwrap();
                }
            }
        }
    }

    return calorie_counts;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
