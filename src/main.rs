mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n = if args.len() > 1 {
        args[1].parse::<i32>().unwrap_or(1)
    } else {
        1
    };

    match n {
        1 => day1::main(),
        2 => day2::main(),
        3 => day3::main(),
        4 => day4::main(),
        _ => println!("Day {} not implemented", n),
    }
}
