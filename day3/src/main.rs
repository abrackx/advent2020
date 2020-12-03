use std::fs;

fn main() {
    let data = fs::read_to_string("./src/data.txt").unwrap();
    let lines = data.lines();
    let part1 = vec![(3, 1)];
    let part2 = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    println!("part 1: {}", run_it(part1, lines.clone()));
    println!("part 2: {}", run_it(part2, lines.clone()));
}

fn hit(line: &str, ind: usize) -> bool {
    if line.chars().nth(ind).unwrap() == '#' {
        return true;
    }
    return false;
}

fn actual_x(ind: usize, len: usize) -> usize {
    let mut x = ind.clone();

    if len <= ind {
        x = ind % len as usize;
    }
    return x;
}

fn run_it(funcs: Vec<(usize, usize)>, lines: core::str::Lines) -> i128 {
    let mut count: i128 = 1;
    funcs.into_iter().for_each(|func| {
        let mut i: usize = 0;
        let mut temp_count = 0;
        let (x, y) = func;
        lines.clone().step_by(y).for_each(|row| {
            if hit(row, actual_x(x * i, row.len())) {
                temp_count += 1;
            }
            i += 1;
        });
        count *= temp_count;
    });

    return count;
}
