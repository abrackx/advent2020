use std::fs;

fn main() {
    let data = fs::read_to_string("./src/data.txt").expect("Unable to parse");
    let lines: Vec<i32> = data.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<i32>) -> bool {
    for x in lines {
        if lines.contains(&(2020 - x)){
            //there must be a better way
            println!("{}", x * lines[lines.iter().position(|y| y == &(2020 - x)).unwrap()]);
            return true;
        }
    }

    return false;
}

fn part2(lines: &Vec<i32>) -> bool {
    for i in 0..lines.len() - 1 {
        let mut sum = Vec::new();
        for j in i+1..lines.len() {
            let x = 2020-(lines[i] + lines[j]);
            if sum.contains(&x) {
                println!("{}", x * lines[i] * lines[j]);
                return true;
            } else {
                sum.push(lines[j]);
            }
        }
    }
    return false;
}