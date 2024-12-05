

use std::fs;
use std::collections::HashSet;

fn check_parents(index: usize, page: i32, list: &Vec<HashSet<i32>>, updates: &mut Vec<i32>) -> bool {
    let mut flag = false;
    for i in 0..index {
        if list[page as usize].contains(&updates[i as usize]) {
            let temp = updates[i as usize];
            updates[i as usize] = updates[index];
            updates[index as usize] = temp;
            flag = true;
        }
    }
    return flag;
}

fn main() {
    let input = fs::read_to_string("src/input2.txt").unwrap();
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let rules = sections[0].split("\n").collect::<Vec<&str>>();
    let section_one = rules.iter().map(|x| x.split("|").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    let section_two = sections[1].split("\n").collect::<Vec<&str>>();
    let updates_string = section_two.iter().map(|x| x.split(",").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    let mut updates = updates_string.into_iter().map(|x| x.iter().map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    let rules = section_one.iter().map(|x| x.iter().map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    let mut min_page = i32::MAX;
    let mut max_page = i32::MIN;
    for i in 0..rules.len() {
        min_page = min_page.min(rules[i][0].min(rules[i][1]));
        max_page = max_page.max(rules[i][0].max(rules[i][1]));
    }
    let mut in_degree = vec![0; (max_page + 1) as usize];
    let mut list = vec![HashSet::new(); (max_page + 1) as usize];
    for i in 0..rules.len() {
        in_degree[rules[i][1] as usize] += 1;
        list[rules[i][0] as usize].insert(rules[i][1]);
    }
 
    let mut sum = 0;
    for i in 0..updates.len() {
        let mut flag = true;
        for j in 0..updates[i].len() {
            if check_parents(j, updates[i][j], &list, &mut updates[i]) {
                flag = false;
            }
        }
        if flag == false {
            println!("{:?}", updates[i]);
            sum += updates[i][updates[i].len() / 2]; 
        }
    }
    println!("{:?}", sum);



}
