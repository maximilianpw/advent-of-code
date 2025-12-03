use std::{fs, ops::Div};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("couldn't read file");

    let mut position: i32 = 50;
    let mut password: i32 = 0;

    for line in contents.lines() {
        let distance: i32 = line[1..].parse().unwrap();

        match line.chars().next().unwrap() {
            'R' => {
                let old_position = position;
                position += distance
            }
            'L' => {
                let old_position = position;
                position -= distance
            }
            _ => panic!("unexpected direction"),
        }

        password += position.div(100);
        position = position.rem_euclid(100);
        // if position == 0 {
        //     password += 1;
        // }
    }

    println!("password: {}", password);
}
