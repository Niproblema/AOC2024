use std::collections::HashMap;
use std::collections::HashSet;
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
    let mut occurrences: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let y_max = input.lines().count();
    let x_max = input.lines().next().unwrap().chars().count();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                occurrences.entry(c).or_default().push((x as i32, y as i32));
            }
        }
    }

    let mut points: HashSet<(i32, i32)> = HashSet::new();

    for (_, collection) in occurrences {
        for i in 0..collection.len() {
            for j in i + 1..collection.len() {
                let (x1, y1) = collection[i];
                let (x2, y2) = collection[j];

                let dx = x2 - x1;
                let dy = y2 - y1;

                let (px1, py1) = (x2 + dx, y2 + dy);
                let (px2, py2) = (x1 - dx, y1 - dy);

                if px1 >= 0 && px1 < x_max as i32 && py1 >= 0 && py1 < y_max as i32 {
                    points.insert((px1, py1));
                    //println!("{} {}", px1, py2);
                }

                if px2 >= 0 && px2 < x_max as i32 && py2 >= 0 && py2 < y_max as i32 {
                    points.insert((px2, py2));
                    //println!("{} {}", px2, py2);
                }
            }
        }
    }

    points.len() as i32
}

fn task_2(input: &str) -> i32 {
    let mut occurrences: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let y_max = input.lines().count();
    let x_max = input.lines().next().unwrap().chars().count();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                occurrences.entry(c).or_default().push((x as i32, y as i32));
            }
        }
    }

    let mut points: HashSet<(i32, i32)> = HashSet::new();

    for (_, collection) in occurrences {
        for i in 0..collection.len() {
            for j in i + 1..collection.len() {
                let (x1, y1) = collection[i];
                let (x2, y2) = collection[j];

                let dx = x2 - x1;
                let dy = y2 - y1;

                let mut mul = 1;
                loop {
                    let (px, py) = (x1 + (dx * mul), y1 + (dy * mul));
                    if px >= 0 && px < x_max as i32 && py >= 0 && py < y_max as i32 {
                        points.insert((px, py));
                    } else {
                        break;
                    }

                    mul += 1;
                }

                mul = 1;
                loop {
                    let (px, py) = (x2 - (dx * mul), y2 - (dy * mul));
                    if px >= 0 && px < x_max as i32 && py >= 0 && py < y_max as i32 {
                        points.insert((px, py));
                    } else {
                        break;
                    }

                    mul += 1;
                }
            }
        }
    }

    points.len() as i32
}
