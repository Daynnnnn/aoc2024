static mut CENTERS: Vec<[usize; 2]> = Vec::new();

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

            if charater != b'A' {
                continue;
            }
            
            if x_index == 0 || y_index == 0 || x_index == x_chars - 1 || y_index == y_chars - 1 {
                continue;
            }

            let check_forward = check(&lines, [[x_index - 1, y_index + 1], [x_index, y_index], [x_index + 1, y_index - 1]]);

            let check_back = check(&lines, [[x_index - 1, y_index - 1], [x_index, y_index], [x_index + 1, y_index + 1]]);

            if check_forward && check_back {
                unsafe { CENTERS.push([x_index, y_index]); }
            }
        }
    }

    unsafe {
        CENTERS.sort();
        CENTERS.dedup();

        println!("{:?}", CENTERS.len());
    }
    
}

fn check(lines: &Vec<&str>, coords: [[usize; 2]; 3]) -> bool {
    let mut string: String = "".to_owned();

    for i in 0..coords.len() {
        let coord = coords[i];

        let line = lines[coord[1]];
        let charater = line.as_bytes()[coord[0]];

        string.push(charater as char);
    }

    if string != "MAS" && string != "SAM" {
        return false;
    }

    return true;
}