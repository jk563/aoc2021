use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn load_from_file(file_path: &str) -> Vec<i64> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    return numbers
}

fn main() {
    let numbers = load_from_file("numbers.txt");

    let zip = numbers
        .iter()
        .skip(1)
        .zip(numbers.iter())
        .fold(0, |acc, (next, current)| if next > current { acc + 1 } else { acc });

    println!("Larger Measurements: {:?}", zip);
}
