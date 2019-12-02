use std::fs;
use std::iter;


fn fuel_required(mass: i32) -> i32 {
    let fuel_required = |x: i32| x / 3 - 2;
    return iter::successors(Some(mass), |x| Some(fuel_required(*x)))
        .skip(1)
        .take_while(|n| n.is_positive())
        .sum();
}


fn main() {
    let filename = "../day1/input.txt";
    let inputs = fs::read_to_string(filename)
        .expect("Something went wrong reading file.");

    let result: i32 = inputs
        .split('\n')
        .filter_map(|n| n.parse::<i32>().ok())
        .map(fuel_required)
        .sum();

    assert_eq!(2, fuel_required(14));
    assert_eq!(966, fuel_required(1969));
    assert_eq!(50346, fuel_required(100756));
    println!("result {}", result);
}
