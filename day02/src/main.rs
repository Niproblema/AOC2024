use std::fs::read_to_string;

fn main() {
    let input_path = "./inputs/input.txt";

    let input = read_to_string(input_path).unwrap();

    let result1 = task_1(&input);
    let result2 = task_2(&input);

    println!("{:?}", result1);
    println!("{:?}", result2);
}

fn task_1(input: &str) -> i32 {
    let mut safe_count = 0;

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if is_safe(&numbers, false) {
            safe_count += 1;
        }
    }

    safe_count
}

fn task_2(input: &str) -> i32 {
    let mut safe_count = 0;

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if is_safe(&numbers, true) {
            safe_count += 1;
        }
    }

    safe_count
}

fn is_safe(input: &Vec<i32>, allow_skip: bool) -> bool {
    if input.len() == 1 {
        return true;
    }

    let mut unsafe_spots = Vec::new();
    let mut dir_set = false;
    let mut direction = false;
    for (i, window) in input.windows(2).enumerate() {
        let (num, next_num) = (window[0], window[1]);

        if (num - next_num).abs() > 3 || (num - next_num).abs() == 0 {
            unsafe_spots.push(i);
            unsafe_spots.push(i + 1);

            if !allow_skip {
                return false;
            } else {
                continue;
            }
        }

        if !dir_set {
            direction = num < next_num;
            dir_set = true;
        } else if (num < next_num) != direction {
            unsafe_spots.push(i);
            unsafe_spots.push(i + 1);
            dir_set = false;

            if !allow_skip {
                return false;
            } else {
                continue;
            }
        }
    }

    if unsafe_spots.is_empty() {
        return true;
    }

    // for i in 0..input.len() {
    //     let mut new_input1 = input.clone();
    //     new_input1.remove(i);
    //     if is_safe(&new_input1, false) {
    //         return true;
    //     }
    // }

    for spot in unsafe_spots.iter() {
        let mut new_input1 = input.clone();
        new_input1.remove(*spot);
        if is_safe(&new_input1, false) {
            return true;
        }
    }

    false
}
