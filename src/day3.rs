use regex::Regex;

pub fn main() {    
    let input = std::fs::read_to_string("input/3.txt").unwrap();

    let regex = Regex::new(r"mul\((?<x>[0-9]*),(?<y>[0-9]*)\)|do\(\)|don't\(\)").unwrap();

    let mut mul_enabled = true;

    let all_mul_calls: String = regex.find_iter(&input.replace('\n', ""))
        .map(|m| {
            return m.as_str().to_string();
        })
        .filter(|m| {
            let matched_command = m.as_str();

            if matched_command == "don't()" {
                mul_enabled = false;
                return false;
            } else if matched_command == "do()" {
                mul_enabled = true;
                return false;
            }

            return mul_enabled;
        }).collect();

    let mul_results: Vec<usize> = regex.captures_iter(&all_mul_calls)
        .map(|captures| {
            let x = captures.name("x").unwrap().as_str().parse::<usize>().unwrap();
            let y = captures.name("y").unwrap().as_str().parse::<usize>().unwrap();

            return x * y;
        })
        .collect();


    let mut total = 0;
    
    for i in 0..mul_results.len() {
        total = total + mul_results[i];
    }

    println!("{}", total)
}