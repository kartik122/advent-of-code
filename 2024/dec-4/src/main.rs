use std::fs;

const TREE: [char; 4] = ['X', 'M', 'A', 'S'];

fn parse_horizontal(i: usize, j: usize, xmas_array: &Vec<Vec<char>>) -> usize {
    let mut ans = 0;
    let mut p = 0;

    // Check leftward direction
    let mut k = i as isize;
    while k >= 0 {
        if p < TREE.len() && xmas_array[k as usize][j] == TREE[p] {
            p += 1;
        } else {
            break;
        }
        if p == TREE.len() {
            ans += 1;
            break;
        }
        k -= 1;
    }

    // Check rightward direction
    p = 0;
    let mut k = i;
    while k < xmas_array.len() {
        if p < TREE.len() && xmas_array[k][j] == TREE[p] {
            p += 1;
        } else {
            break;
        }
        if p == TREE.len() {
            ans += 1;
            break;
        }
        k += 1;
    }

    ans
}

fn parse_vertical(i: usize, j: usize, xmas_array: &Vec<Vec<char>>) -> usize {
    let mut ans = 0;
    let mut p = 0;

    // Check upward direction
    let mut k = j as isize;
    while k >= 0 {
        if p < TREE.len() && xmas_array[i][k as usize] == TREE[p] {
            p += 1;
        } else {
            break;
        }
        if p == TREE.len() {
            ans += 1;
            break;
        }
        k -= 1;
    }

    // Check downward direction
    p = 0;
    let mut k = j;
    while k < xmas_array[0].len() {
        if p < TREE.len() && xmas_array[i][k] == TREE[p] {
            p += 1;
        } else {
            break;
        }
        if p == TREE.len() {
            ans += 1;
            break;
        }
        k += 1;
    }

    ans
}

fn parse_pos_diagonal(i: usize, j: usize, xmas_array: &Vec<Vec<char>>) -> usize {
    let mut ans = 0;
    let mut p = 0;

    // Check upward-right diagonal
    let (mut x, mut y) = (i as isize, j as isize);
    while x >= 0 && y < xmas_array[0].len() as isize {
        if p < TREE.len() && xmas_array[x as usize][y as usize] == TREE[p] {
            p += 1;
            x -= 1;
            y += 1;
        } else {
            break;
        }
        if p == TREE.len() {
            ans += 1;
            break;
        }
    }

    // Check downward-left diagonal
    p = 0;
    let (mut x, mut y) = (i as isize, j as isize);
    while x < xmas_array.len() as isize && y >= 0 {
        if p < TREE.len() && xmas_array[x as usize][y as usize] == TREE[p] {
            p += 1;
            x += 1;
            y -= 1;
        } else {
            break;
        }
        if p == TREE.len() {
            ans += 1;
            break;
        }
    }

    ans
}

fn parse_neg_diagonal(i: usize, j: usize, xmas_array: &Vec<Vec<char>>) -> usize {
    let mut ans = 0;
    let mut p = 0;

    // Check upward-left diagonal
    let (mut x, mut y) = (i as isize, j as isize);
    while x >= 0 && y >= 0 {
        if p < TREE.len() && xmas_array[x as usize][y as usize] == TREE[p] {
            p += 1;
            x -= 1;
            y -= 1;
        } else {
            break;
        }
        if p == TREE.len() {
            ans += 1;
            break;
        }
    }

    // Check downward-right diagonal
    p = 0;
    let (mut x, mut y) = (i as isize, j as isize);
    while x < xmas_array.len() as isize && y < xmas_array[0].len() as isize {
        if p < TREE.len() && xmas_array[x as usize][y as usize] == TREE[p] {
            p += 1;
            x += 1;
            y += 1;
        } else {
            break;
        }
        if p == TREE.len() {
            ans += 1;
            break;
        }
    }

    ans
}

fn parse_tree(i: usize, j: usize, xmas_array: &Vec<Vec<char>>) -> usize {
    let mut ans = 0;
    ans += parse_horizontal(i, j, xmas_array);
    ans += parse_vertical(i, j, xmas_array);
    ans += parse_pos_diagonal(i, j, xmas_array);
    ans += parse_neg_diagonal(i, j, xmas_array);
    return ans;
}
fn parse_diag(i: usize, j: usize, xmas_array: &Vec<Vec<char>>) -> usize {
    let mut ans = 0;
    let x = i as isize;
    let y = j as isize;
    if (x > 0 && y < (xmas_array[0].len() - 1) as isize && xmas_array[(x - 1) as usize][(y + 1) as usize] == 'M' && x < (xmas_array.len() - 1) as isize && y > 0 as isize && xmas_array[(x + 1) as usize][(y - 1) as usize] == 'S') ||  (x > 0 && y < (xmas_array[0].len() - 1) as isize && xmas_array[(x - 1) as usize][(y + 1) as usize] == 'S' && x + 1 < xmas_array.len() as isize && y > 0 as isize && xmas_array[(x + 1) as usize][(y - 1) as usize] == 'M'){
        ans += 1;
    }
    ans
}

fn parse_rev(i: usize, j: usize, xmas_array: &Vec<Vec<char>>) -> usize {
    let mut ans = 0;
    let x = i as isize;
    let y = j as isize;
    if (x > 0 && y > 0 && xmas_array[(x - 1) as usize][(y - 1) as usize] == 'M' && x < (xmas_array.len() - 1) as isize && y < (xmas_array[0].len() - 1) as isize && xmas_array[(x + 1) as usize][(y + 1) as usize] == 'S') ||  (x > 0 && y > 0 && xmas_array[(x - 1) as usize][(y - 1) as usize] == 'S' && x + 1 < xmas_array.len() as isize && y < (xmas_array[0].len() - 1) as isize && xmas_array[(x + 1) as usize][(y + 1) as usize] == 'M'){
        ans += 1;
    }
    ans
}

fn new_parse_tree(i: usize, j: usize, xmas_array: &Vec<Vec<char>>) -> bool {
    let pos = parse_diag(i, j, xmas_array);
    let neg = parse_rev(i, j, xmas_array);
    return pos == 1 && neg == 1;
}


fn main() {
    let input = fs::read_to_string("advent-of-code/2024/dec-4/src/input2.txt").unwrap();
    let mut xmas_array: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let line_vec: Vec<char> = line.chars().collect();
        xmas_array.push(line_vec);
    }
    let mut parse_array: Vec<(usize, usize)> = Vec::new();
    // for i in 0..xmas_array.len() {
    //     for j in 0..xmas_array[i].len() {
    //         if xmas_array[i][j] == 'X' {
    //             parse_array.push((i, j));
    //         }
    //     }
    // }
    // let mut ans = 0;
    // while parse_array.len() > 0 {
    //     let (i, j) = parse_array.remove(0);
    //     ans += parse_tree(i, j, &xmas_array);
    // }
    for i in 0..xmas_array.len() {
        for j in 0..xmas_array[i].len() {
            if xmas_array[i][j] == 'A' {
                parse_array.push((i, j));
            }
        }
    }
    let mut ans = 0;
    while parse_array.len() > 0 {
        let (i, j) = parse_array.remove(0);
        if new_parse_tree(i, j, &xmas_array) {
            ans += 1;
        }
    }
    println!("{}", ans);
    println!("Hello, world!");
}
