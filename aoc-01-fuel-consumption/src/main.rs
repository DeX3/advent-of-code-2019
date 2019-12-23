use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mass = line.parse::<i32>().unwrap();
        let fuel = calculate_fuel_for_mass(mass);

        sum += fuel;
    }

    println!("Total fuel needed: {}", sum);
}

fn calculate_fuel_for_mass(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;

    if fuel > 0 {
        return fuel + calculate_fuel_for_mass(fuel);
    } else {
        return 0;
    }
}
