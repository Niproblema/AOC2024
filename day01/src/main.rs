use std::fs::read_to_string;

fn main() {
    let input_path = "./inputs/input.txt";

    let input = read_to_string(input_path).unwrap();

    let result1 = task_1(&input);
    let result2 = task_2(&input);

    println!("{:?}", result1);
    println!("{:?}", result2);
}

fn parse_number_pairs(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();
    left_numbers.reserve(input.lines().count());
    right_numbers.reserve(input.lines().count());

    // Process each line
    for line in input.lines() {
        // Split the line by whitespace
        let numbers: Vec<&str> = line.split_whitespace().collect();

        // Skip invalid lines
        if numbers.len() != 2 {
            continue;
        }

        // Try to parse both numbers
        if let (Ok(left), Ok(right)) = (numbers[0].parse::<i32>(), numbers[1].parse::<i32>()) {
            left_numbers.push(left);
            right_numbers.push(right);
        }
    }

    (left_numbers, right_numbers)
}

fn task_1(input: &str) -> i32 {
    let (mut left_numbers, mut right_numbers) = parse_number_pairs(input);
    left_numbers.sort_unstable();
    right_numbers.sort_unstable();

    let mut delta_sum = 0;
    for (&left, &right) in left_numbers.iter().zip(right_numbers.iter()) {
        if left != right {
            delta_sum += (left - right).abs();
        }
    }

    delta_sum
}

fn task_2(input: &str) -> i32 {
    let (left_numbers, right_numbers) = parse_number_pairs(input);

    let mut right_numbers_map = std::collections::HashMap::new();
    for &right in right_numbers.iter() {
        *right_numbers_map.entry(right).or_insert(0) += 1;
    }

    let mut sim_sum = 0;
    for &left in left_numbers.iter() {
        if let Some(count) = right_numbers_map.get_mut(&left) {
            sim_sum += left * *count;
        }
    }

    sim_sum
}
