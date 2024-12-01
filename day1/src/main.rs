use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let (mut left, mut right) = read_input();

    left.sort();
    right.sort();

    let mut sum = 0;

    for (l, r) in left.iter().zip(right.iter()) {
        let diff = (l - r).abs();
        sum += diff;
    }

    println!("SUM: {sum}");

    // ==================================
    let mut counts = HashMap::new();

    for n in right {
        counts.entry(n).and_modify(|c| *c += 1).or_insert(1_i32);
    }

    let mut sum2 = 0;

    for n in left {
        sum2 += n * counts.get(&n).unwrap_or(&0);
    }

    println!("SUM2: {sum2}");
}

fn read_input() -> (Vec<i32>, Vec<i32>) {
    let mut left_list = vec![];
    let mut right_list = vec![];

    let input = read_to_string("input.txt").unwrap();

    for line in input.split("\n") {
        let parts: Vec<_> = line.split("   ").collect();
        assert_eq!(parts.len(), 2);

        let left = parts[0].parse::<i32>().unwrap();
        let right = parts[1].parse::<i32>().unwrap();

        left_list.push(left);
        right_list.push(right);
    }

    (left_list, right_list)
}
