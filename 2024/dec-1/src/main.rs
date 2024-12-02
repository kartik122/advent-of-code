use std::collections::HashMap;

fn calculate_difference(mut left_map: HashMap<i32, i32>, mut right_map: HashMap<i32, i32>) -> i32 {
    let mut left_vec: Vec<(i32, i32)> = left_map.into_iter().collect();
    let mut right_vec: Vec<(i32, i32)> = right_map.into_iter().collect();
    left_vec.sort_by_key(|&(key, _)| key);
    right_vec.sort_by_key(|&(key, _)| key);

    let mut i = 0;
    let mut j = 0;
    let mut total_difference = 0;

    while i < left_vec.len() && j < right_vec.len() {
        let (left_key, left_value) = left_vec[i];
        let (right_key, right_value) = right_vec[j];

        let difference = (left_key - right_key).abs();
        let weight = left_value.min(right_value);
        total_difference += difference * weight;

        left_vec[i].1 -= weight;
        right_vec[j].1 -= weight;

        if left_vec[i].1 == 0 {
            i += 1;
        }
        if right_vec[j].1 == 0 {
            j += 1;
        }
    }

    total_difference
}

fn split_and_count(input: &str) -> (HashMap<i32, i32>, HashMap<i32, i32>) {
    let mut left_map = HashMap::new();
    let mut right_map = HashMap::new();

    let mut split_values = input.split_whitespace();
    
    while let (Some(left), Some(right)) = (split_values.next(), split_values.next()) {
        if let (Ok(left_num), Ok(right_num)) = (left.parse::<i32>(), right.parse::<i32>()) {
            *left_map.entry(left_num).or_insert(0) += 1;
            *right_map.entry(right_num).or_insert(0) += 1;
        }
    }

    (left_map, right_map)
}

fn multiply_and_add(left_map: HashMap<i32, i32>, right_map: HashMap<i32, i32>) -> i32 {
    left_map
        .iter()
        .map(|(key, left_value)| {
            let right_value = right_map.get(key).copied().unwrap_or(0);
            key * right_value * left_value
        })
        .sum()
}
fn main() {
    println!("Hello, world!");
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let input: &str = &contents;
    let (left_map, right_map) = split_and_count(input);
    // print!("{}",calculate_difference(left_map, right_map));

    print!("{}", multiply_and_add(left_map, right_map));
    
    // print!("{}",sum);

}
