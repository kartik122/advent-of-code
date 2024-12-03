use std::fs;
use regex::Regex;
use std::collections::VecDeque as Vec_Deque;
fn main() {
    let input = fs::read_to_string("advent-of-code/2024/dec-3/src/input2.txt").unwrap();
    let regex_pattern = r"mul\(\d+,\d+\)";
    let regex_pattern_do = r"do\(\)";
    let regex_pattern_dont = r"don\'t\(\)";
    let regex = Regex::new(regex_pattern).unwrap();
    let regex_do = Regex::new(regex_pattern_do).unwrap();
    let regex_dont = Regex::new(regex_pattern_dont).unwrap();
    let mut sum = 0;
    let mut stack_do: Vec_Deque<i32> = Vec_Deque::new();
    let mut stack_dont: Vec_Deque<i32> = Vec_Deque::new();
    let mut stack_match: Vec_Deque<(i32, i32, usize)> = Vec_Deque::new();
    for regex_do in regex_do.find_iter(&input) {
        stack_do.push_back(i32::try_from(regex_do.start()).unwrap());
    }
    for regex_dont in regex_dont.find_iter(&input) {
        stack_dont.push_back(i32::try_from(regex_dont.start()).unwrap());
    }
    for mat in regex.find_iter(&input) {
        let str = mat.as_str();
        let splitter = str.split(",").collect::<Vec<&str>>();  
        let num1 = splitter[0][4..].parse::<i32>().unwrap();
        let num2: i32 = splitter[1][0..splitter[1].len()-1].parse::<i32>().unwrap();
        stack_match.push_back((num1, num2, mat.start()));
    }
    let mut j: usize = 0;
    let mut k: usize= 0;
    for i in 0..stack_match.len() {
        let stack_matched = i32::try_from(stack_match[i].2).unwrap();
        while j < stack_do.len() && stack_matched > stack_do[j] {
            j += 1;
        }
        while k < stack_dont.len() && stack_matched > stack_dont[k] {
            k += 1;
        }
        let mut diff1 = i32::MAX;
        let mut diff2 = i32::MAX;
        if j != 0 {
            diff1 = stack_matched - stack_do[j-1];
        }
        if k != 0 {
            diff2 = stack_matched - stack_dont[k-1];
        }
        if diff1 == i32::MAX && diff2 == i32::MAX {
            if j < stack_do.len() && k < stack_dont.len() {
            print!("----- {:?} {:?} {:?} \n", stack_matched, stack_do[j], stack_dont[k]);
            }
            sum += stack_match[i].0 * stack_match[i].1;
        }
        else if diff1 > 0 && diff2 > 0 {
            if diff1 < diff2 {
                sum += stack_match[i].0 * stack_match[i].1;
            }
            
        }
       
    }
    
   
    print!("{}", sum);

}
