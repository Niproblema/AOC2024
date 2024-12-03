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
    let mut sum = 0;

    let mut i = 0;
    let data = input.as_bytes();

    while i < data.len() {
        if !data[i..].starts_with(b"mul(") {
            i += 1;
            continue;
        }
        i += 4;

        let mut num1 = 0;
        let mut num2 = 0;

        while data[i].is_ascii_digit() {
            num1 = 10 * num1 + (data[i] - b'0') as i32;
            i += 1;
        }

        if data[i] != b',' {
            continue;
        }
        i += 1;

        while data[i].is_ascii_digit() {
            num2 = 10 * num2 + (data[i] - b'0') as i32;
            i += 1;
        }

        if data[i] != b')' {
            continue;
        }

        sum += num1 * num2;

        i += 1;
    }

    sum
}

fn task_2(input: &str) -> i32 {
    let mut sum = 0;
    let mut enabled = true;
    let mut i = 0;
    let data = input.as_bytes();

    while i < data.len() {
        if !enabled && data[i..].starts_with(b"do()") {
            enabled = true;
            i += 3;
        } else if enabled && data[i..].starts_with(b"don't()") {
            enabled = false;
            i += 6;
        } else if enabled && data[i..].starts_with(b"mul(") {
            i += 4;

            let mut num1 = 0;
            let mut num2 = 0;

            while data[i].is_ascii_digit() {
                num1 = 10 * num1 + (data[i] - b'0') as i32;
                i += 1;
            }

            if data[i] != b',' {
                continue;
            }
            i += 1;

            while data[i].is_ascii_digit() {
                num2 = 10 * num2 + (data[i] - b'0') as i32;
                i += 1;
            }

            if data[i] != b')' {
                continue;
            }

            sum += num1 * num2;
        }

        i += 1;
    }

    sum
}
