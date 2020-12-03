use std::fs;

fn main() {
    let data = fs::read_to_string("./src/data.txt").unwrap();
    let lines = data.lines();
    let mut i: usize = 0;
    let mut count1 = 0;
    let mut count2: i128 = 1;
    lines.clone().into_iter().for_each(|row| {
        if part1(row, actual_x(3 * i, row.len())) {
            count1 += 1;
        }
        i += 1;
    });

    let funcs = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    funcs.into_iter().for_each(|func| {
        let mut k: usize = 0;
        let mut temp_count = 0;
        let (x, y) = func;
        lines.clone().step_by(y).for_each(|row| {
            if part1(row, actual_x(x * k, row.len())) {
                temp_count += 1;
            }
            k += 1;
        });
        count2 *= temp_count;
    });

    println!("part 1: {}", count1);
    println!("part 2: {}", count2);
}

fn part1(line: &str, ind: usize) -> bool {
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
