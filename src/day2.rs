pub fn main() {
    let start = std::time::Instant::now();
    
    let input = std::fs::read_to_string("input/2.txt").unwrap();
    let reports = input.lines().map(|s| s.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    let number_of_safe_reports = reports.iter().filter(|report| check_report(report, true)).count();
    
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
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
