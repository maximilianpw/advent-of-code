use std::fs;

fn main() {
    let file: String = fs::read_to_string("init-input.txt").expect("Unable to read file");
    let item_ranges: Vec<&str> = file.split(",").collect();
    let mut total: i32 = 0;

    for item in item_ranges {}
}
