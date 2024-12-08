pub fn main() {
    let input = std::fs::read_to_string("input/4.txt").unwrap();

    let lines: Vec<&str> = input.lines().collect();

    let x_chars = lines.len();
    let y_chars = lines[0].len();

    let mut matches_found = 0;

    for y_index in 0..y_chars {
        let line = lines[y_index];
        for x_index in 0..x_chars {
            let charater = line.as_bytes()[x_index];

            if charater != b'X' {
                continue;
            }
            
            if x_index <= x_chars - 4 {
                let check_right = check(&lines, [[x_index, y_index], [x_index + 1, y_index], [x_index + 2, y_index], [x_index + 3, y_index]]);

                if check_right {
                    matches_found = matches_found + 1;
                }

                if y_index <= y_chars - 4 {
                   let check_down_right = check(&lines, [[x_index, y_index], [x_index + 1, y_index + 1], [x_index + 2, y_index + 2], [x_index + 3, y_index + 3]]);
                    
                    if check_down_right {
                        matches_found = matches_found + 1;
                    }
                }

                if y_index >= 3 {
                    let check_up_right = check(&lines, [[x_index, y_index], [x_index + 1, y_index - 1], [x_index + 2, y_index - 2], [x_index + 3, y_index - 3]]);
                
                    if check_up_right {
                        matches_found = matches_found + 1;
                    }
                }
            }
            
            if x_index >= 3 {
                let check_left = check(&lines, [[x_index, y_index], [x_index - 1, y_index], [x_index - 2, y_index], [x_index - 3, y_index]]);
                
                if check_left {
                    matches_found = matches_found + 1;
                }

                if y_index <= y_chars - 4 {
                    let check_down_left = check(&lines, [[x_index, y_index], [x_index - 1, y_index + 1], [x_index - 2, y_index + 2], [x_index - 3, y_index + 3]]);
                    
                    if check_down_left {
                        matches_found = matches_found + 1;
                    }
                }

                if y_index >= 3 {
                    let check_up_left = check(&lines, [[x_index, y_index], [x_index - 1, y_index - 1], [x_index - 2, y_index - 2], [x_index - 3, y_index - 3]]);
                
                    if check_up_left {
                        matches_found = matches_found + 1;
                    }
                }
            }
            
            if y_index <= y_chars - 4 {
                let check_down = check(&lines, [[x_index, y_index], [x_index, y_index + 1], [x_index, y_index + 2], [x_index, y_index + 3]]);
            
                if check_down {
                    matches_found = matches_found + 1;
                }
            }

            if y_index >= 3 {
                let check_up = check(&lines, [[x_index, y_index], [x_index, y_index - 1], [x_index, y_index - 2], [x_index, y_index - 3]]);
            
                if check_up {
                    matches_found = matches_found + 1;
                }
            }
            
        }
    }

    println!("{}", matches_found);
}

fn check(lines: &Vec<&str>, coords: [[usize; 2]; 4]) -> bool {
    let mut string: String = "".to_owned();

    for i in 0..coords.len() {
        let coord = coords[i];

        let line = lines[coord[1]];
        let charater = line.as_bytes()[coord[0]];

        string.push(charater as char);
    }

    if string == "XMAS" {
        return true;
    }

    return false;
}