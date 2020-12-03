use std::fs;

fn main() {
    let data = fs::read_to_string("./src/data.txt").unwrap();
    let lines = data.lines();
    let mut whatever: Vec<(usize, usize, &str, &str)> = Vec::new();
    lines.into_iter().for_each(|x| whatever.push(split_to_whatever(x)));
    println!("{}", part1(&whatever));
    println!("{}", part2(&whatever));
}

fn part1(list: &Vec<(usize, usize, &str, &str)>) -> i32 {
    let mut total = 0;
    list.into_iter()
        .for_each(|item| {
            let count = item.3.matches(item.2).count();
            if count >= item.0 && count <= item.1 {
                total = total + 1;
            }
        });
    return total;
}

fn part2(list: &Vec<(usize, usize, &str, &str)>) -> i32 {
    let mut total = 0;
    list.into_iter()
        .for_each(|item| {
            if ((&item.3[item.0-1..item.0] == item.2) || (&item.3[item.1-1..item.1] == item.2))
                && (item.3[item.1-1..item.1] != item.3[item.0-1..item.0]) {
                total = total + 1;
            }
        });
    return total;
}

fn split_to_whatever(line: &str) -> (usize, usize, &str, &str) {
    let data: Vec<&str> = line.split(|x| x == '-' || x == ':' || x == ' ').collect();
    let min = data[0].parse::<usize>().unwrap();
    let max = data[1].parse::<usize>().unwrap();
    let target = data[2];
    let pass = data[4]; //idk why 3 is blank but it is
    return (min, max, target, pass);
}

