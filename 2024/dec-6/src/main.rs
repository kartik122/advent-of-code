use std::fs;
use std::collections::HashSet;

fn check_extreme(x: isize, y: isize, x_max: usize, y_max: usize) -> bool {
    if x < 0 || y < 0 || x >= x_max as isize || y >= y_max as isize {
        return true;
    }
    return false;
}
fn main() {
    let input = fs::read_to_string("src/input2.txt").unwrap();
    let dir: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut lab = input.lines().map(|row| row.char_indices().map(|(i, c)| (c)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut mark = vec![vec![0; lab[0].len()]; lab.len()];
   
    let mut current_dir = 0;
    let mut count = 0;
    let mut cx = 0;
    let mut cy = 0;
    for i in 0..lab.len() {
        for j in 0..lab[i].len() {
            if lab[i][j] == '^' {
                mark[i][j] = 1;
                cx = i as isize;
                cy = j as isize;
            }
            else if lab[i][j] == '#' {
                  mark[i][j] = -1; 
            }
        }
    }
    for i in 0..lab.len() {
        for j in 0..lab[0].len() {
            if lab[i][j] == '#' || lab[i][j] == '^' {
                continue;
            }
            let mut current_x = cx;
            let mut current_y = cy;
            let mut seen: HashSet<(isize, isize, i32)> = HashSet::<(isize, isize, i32)>::new();
            lab[i][j] = '#';
            current_dir = 0;
            while current_x >= 0 && current_y >= 0 && current_x < lab.len() as isize && current_y < lab[0].len() as isize && seen.contains(&(current_x, current_y, current_dir)) == false {
                seen.insert((current_x, current_y, current_dir));
                loop {
                    let temp_x = current_x + dir[current_dir as usize].0;
                    let temp_y = current_y + dir[current_dir as usize].1;

                    if temp_x >= 0 && temp_y >= 0 && temp_x < lab.len() as isize && temp_y < lab[0].len() as isize && lab[temp_x as usize][temp_y as usize] == '#' {
                        current_dir = (current_dir + 1) % 4;
                    } else {
                        current_x = temp_x;
                        current_y = temp_y;
                        break;
                    }
                }
            }
            if seen.contains(&(current_x, current_y, current_dir)) {
                count += 1;
            }
            lab[i][j] = '.';
        }
    }
    print!("{}", count);

}

