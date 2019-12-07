use std::collections::HashSet;
use std::fs;
use std::iter;
use std::iter::FromIterator;

type Point2D = (i32, i32);

fn take_step(direction: &char, from: &Point2D) -> Point2D {
    match direction {
        'R' => (from.0 + 1, from.1),
        'U' => (from.0, from.1 + 1),
        'D' => (from.0, from.1 - 1),
        'L' => (from.0 - 1, from.1),
        _ => panic!(format!("unknown direction: {}", direction)),
    }
}

fn points(wire_steps: &Vec<&str>) -> Vec<Point2D> {
    wire_steps
        .iter()
        .flat_map(|step| {
            let direction = step.chars().next().unwrap();
            let n: usize = step[1..].parse().unwrap();
            iter::repeat(direction).take(n)
        })
        .scan((0, 0), |pos, step_dir| {
            *pos = take_step(&step_dir, &pos);
            Some(*pos)
        })
        .collect()
}

fn origin_distance(p: &Point2D) -> i32 {
    p.0.abs() + p.1.abs()
}

fn signal_delay(steps: &Vec<Point2D>, p: &Point2D) -> Option<i32> {
    steps
        .iter()
        .position(|x| x == p)
        .map(|x| x + 1)
        .map(|x| x as i32)
}

fn main() {
    let filename = "../day3/input.txt";
    let file_contents = fs::read_to_string(filename).expect("Something went wrong reading file.");
    let inputs: Vec<Vec<&str>> = file_contents
        .split('\n')
        .map(|line| line.split(',').collect())
        .collect();

    let wire1_points = points(&inputs[0]);
    let wire2_points = points(&inputs[1]);
    let wire1: HashSet<Point2D> = HashSet::from_iter(wire1_points.clone());
    let wire2: HashSet<Point2D> = HashSet::from_iter(wire2_points.clone());

    let common_points = wire1.intersection(&wire2);

    // set intersection of points(wire1) & points(wire2)
    let closest: Point2D = *(common_points.clone())
        .min_by_key(|p| origin_distance(p))
        .unwrap();

    let closest2: i32 = common_points
        .map(|p: &Point2D| {
            vec![&wire1_points, &wire2_points]
                .iter()
                .map(|points| signal_delay(points, &p).unwrap())
                .sum()
        })
        .min()
        .unwrap();

    println!(
        "closest: {:?} which is {} away",
        closest,
        origin_distance(&closest)
    );
    println!("closest part 2: {}", closest2)
}
