fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n = if args.len() > 1 {
        args[1].parse::<i32>().unwrap_or(1)
    } else {
        1
    };

    match n {
        1 => day_1(),
        2 => day_2(),
        _ => println!("Day {} not implemented", n),
    }

}

fn day_1() {
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

// In the example above, the reports can be found safe or unsafe by checking those rules:
// 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
// 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
// 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
// 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
// 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
// 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
//
// The above comments have nothing to do with the actual problem!!!

fn day_2() {
    let input = std::fs::read_to_string("input/2.txt").unwrap();

    let reports = input.lines().map(|s| s.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();

    let number_of_safe_reports = reports.iter().filter(|report| check_report(report, true)).count();

    println!("{}", number_of_safe_reports);
}


fn check_report(report: &Vec<i32>, is_full_report: bool) -> bool {
    let asc = report[0] - report[1] < 0;

    for i in 0..report.len() {
        if i == report.len() - 1 {
            continue;
        }

       let mut diff_to_next = report[i] - report[i + 1];

       if asc && diff_to_next > 0 {
        return is_full_report && problem_dampener_result(report, i);
       } else if !asc && diff_to_next < 0 {
        return is_full_report && problem_dampener_result(report, i);
       } else if diff_to_next == 0 {
        return is_full_report && problem_dampener_result(report, i);
       }

       if diff_to_next < 0 {
         diff_to_next = -diff_to_next;
       }

       if diff_to_next > 3 {
        return is_full_report && problem_dampener_result(report, i);
       }
    }

    return true;
}

fn problem_dampener_result(report: &Vec<i32>, index: usize) -> bool {
    let mut report_with_anomoly_removed = report.clone();
    report_with_anomoly_removed.remove(index);

    if check_report(&report_with_anomoly_removed, false) == true {
        return true;
    }

    if index != report.len() - 1 {
        let mut report_with_anomoly_removed = report.clone();
        report_with_anomoly_removed.remove(index + 1);
    
        if check_report(&report_with_anomoly_removed, false) == true {
            return true;
        }
    }


    if index != 0 {
        let mut report_with_anomoly_removed = report.clone();
        report_with_anomoly_removed.remove(index - 1);
    
        if check_report(&report_with_anomoly_removed, false) == true {
            return true;
        }
    }

    return false;
}