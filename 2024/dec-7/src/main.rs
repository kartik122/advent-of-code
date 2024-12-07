use std::fs;
use std::collections::HashMap;

fn get_recur(
    operations: &Vec<i64>,
    index: usize,
    sum: i64,
    dp: &mut HashMap<usize, HashMap<i64, bool>>,
    curr_sum: i64,
) -> bool {
    if index == operations.len() {
        // Insert the result into dp
        dp.entry(index)
            .or_insert_with(HashMap::new)
            .insert(curr_sum, curr_sum == sum);
        return curr_sum == sum;
    }

    // Check if the value already exists in dp
    if let Some(inner_map) = dp.get(&index) {
        if let Some(&result) = inner_map.get(&curr_sum) {
            return result;
        }
    }

    // Compute and store the result
    let join_sum = format!("{}{}", curr_sum, operations[index]);
    let concat_sum = join_sum.parse::<i64>().unwrap();
    
    let result = get_recur(operations, index + 1, sum, dp, curr_sum + operations[index])
        || get_recur(operations, index + 1, sum, dp, curr_sum * operations[index]) || get_recur(operations, index + 1, sum, dp, concat_sum);

    dp.entry(index)
        .or_insert_with(HashMap::new)
        .insert(curr_sum, result);

    return result;
}
fn main() {
    let input = fs::read_to_string("src/input2.txt").unwrap();
    let tests = input.lines().collect::<Vec<&str>>();
    let mut sum: i64 = 0;
    for test in tests {
        let test_result: i64 = test.split(":").collect::<Vec<&str>>()[0].parse::<i64>().unwrap();
        let operations = test.split(":").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>().into_iter().map(|x| x.parse::<i64>().unwrap_or_default()).collect::<Vec<i64>>();
        let mut dp: HashMap<usize, HashMap<i64, bool>> = HashMap::new();
        if get_recur(&operations, 2, test_result, &mut dp, operations[1]) {
            sum += test_result;
        }
    }
    print!("{}", sum);
}
 
