use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read file");
    let item_ranges: Vec<&str> = file.trim().split(',').collect();
    let mut total: i64 = 0;

    for full_item in item_ranges {
        let mut bounds = full_item.split("-");
        let start: i64 = bounds.next().unwrap().parse().unwrap();
        let end: i64 = bounds.next().unwrap().parse().unwrap();

        for number in start..(end + 1) {
            let length = number.to_string().chars().count();

            let chars: Vec<char> = number.to_string().chars().collect();
            let mut part_one: String = chars[..length / 2].iter().collect();
            let mut part_two: String = chars[length / 2..].iter().collect();

            let mut groups: Vec<String> = Vec::new();

            for i in 0..chars.len() - 1 {
                let pair = format!("{}{}", chars[i], chars[i + 1]);
                groups.push(pair);
            }

            if part_one == part_two {
                let matcher = format!("{part_one}{part_two}").parse::<i64>().unwrap();
                println!("match! on: {matcher}");
                total += matcher
            }
        }
    }

    println!("Total: {}", total);
}
