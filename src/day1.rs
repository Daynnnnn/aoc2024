pub fn main() {
    let input = std::fs::read_to_string("input/1.txt").unwrap();
    
    let mut numbers_before_tab = input.lines().map(|s| s.split_whitespace().nth(0).unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut numbers_after_tab = input.lines().map(|s| s.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();

    numbers_before_tab.sort();
    numbers_after_tab.sort();

    if numbers_before_tab.len() != numbers_after_tab.len() {
        println!("Error: numbersBeforeTab and numbersAfterTab have different lengths");
        return;
    }

    let mut sum = 0;

    for i in 0..numbers_before_tab.len() {
        let mut range = numbers_before_tab[i] - numbers_after_tab[i];

        if range < 0 {
            range = -range;
        }

        sum += range;
    }

    println!("{}", sum);
}
