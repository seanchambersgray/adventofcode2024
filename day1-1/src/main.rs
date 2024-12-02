use std::fs::read_to_string;

fn main() {
    let input = read_lines("input");

    let mut column1 = vec![];
    let mut column2 = vec![];
    for line in input {
        let mut iter = line.split_whitespace();
        column1.push(iter.next().unwrap().parse::<u64>().unwrap());
        column2.push(iter.next().unwrap().parse::<u64>().unwrap());
    }

    column1.sort();
    column2.sort();

    let mut running_total: u64 = 0;

    for i in 0..column1.len() {
        let difference = column1[i].abs_diff(column2[i]);
        running_total += difference;
    }

    println!("{running_total}");
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
