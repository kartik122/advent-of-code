use std::fs;

fn check(report: Vec<i32>, i: usize) -> bool {
    print!("IN THE FUNCTION : {} {} {} \n", report[i], report[i+1], report[i+2]);
    let diff: i32 = report[i] - report[i+1];
    if diff.abs() < 1 || diff.abs() > 3 {
        return false;
    }
    let mut flag: bool = true;
    let mut j: usize = i;
    while j < report.len() - 1 {
        print!("IN THE LOOP : {} {} {} \n", report[j], report[j+1], diff);
        let new_diff: i32 = report[j] - report[j+1];
        if new_diff.abs() < 1 || new_diff.abs() > 3 || new_diff * diff < 0 {
            print!("BREAKING \n {} +++++ {} ----- {} ===== {} \n", report[j], report[j+1], diff, new_diff);
            print!("BREAKING \n");
            flag = false;
            break;
        }
        j += 1;
    }
    flag
}
fn main() {
    let input_file   = fs::read_to_string("/Users/kargupta8/Desktop/advent-of-code/dec2/src/input2.txt").unwrap();
    let mut safe: i32 = 0;
    for line in input_file.lines() {
        let report: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        let mut v1 = report.clone();
        v1.remove(0);
        let mut v2 = report.clone();
        v2.remove(1);
        let initial_flag = check(v1, 0) || check(v2, 0);
        if initial_flag == true {
            print!("{} \n", line);
            safe += 1;
        }
        else {
            let diff: i32 = report[0] - report[1];
            let mut flag = true;
            let mut i: usize = 0;
            while i < report.len() - 1 {
                let new_diff: i32 = report[i] - report[i+1];
                if new_diff.abs() < 1 || new_diff.abs() > 3 || new_diff * diff < 0  {
                    let mut vec1 = report.clone();
                    let mut vec2 = report.clone();
                    vec1.remove(i);
                    vec2.remove(i+1);
                    flag = check(vec1, 0) || check(vec2, 0);
                    break;
                }
                i += 1;
            }
            if flag == true {
                safe += 1;
            } else {
                print!("UNSAFE = {} \n", line);
            }
        }
    }
    println!("Safe: {}", safe);
}
