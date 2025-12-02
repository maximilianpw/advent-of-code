use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("couldn't read file");

    let mut position: i32 = 50;
    let mut password: i32 = 0;

    for line in contents.lines() {
        let distance: i32 = line[1..].parse().unwrap();

        match line.chars().next().unwrap() {
            'R' => position += distance,
            'L' => position -= distance,
            _ => panic!("unexpected direction"),
        }

        position = position.rem_euclid(100);
        if position == 0 {
            password += 1;
        }
    }

    println!("password: {}", password);
}
