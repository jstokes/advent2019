use std::fs;

fn read_word(opcodes: &Vec<i32>, start_idx: usize) -> (i32, i32, i32) {
    (opcodes[opcodes[start_idx + 1] as usize],
     opcodes[opcodes[start_idx + 2] as usize],
     opcodes[start_idx + 3])
}

fn run_program(opcodes: &mut Vec<i32>) {
    let mut curr_idx: usize = 0;
    loop {
        let op = opcodes[curr_idx];
        if op == 99 { break };
        let (x, y, update_pos) = read_word(opcodes, curr_idx);
        let new_value = match op {
            1 => x + y,
            2 => x * y,
            x => panic!(format!("unknown opcode: {}", x))
        };
        opcodes[update_pos as usize] = new_value;
        curr_idx += 4;
    }
}


fn main() {

    assert_eq!(
        vec![3500,9,10,70,2,3,11,0,99,30,40,50],
        {
            let mut input: Vec<i32> = vec![1,9,10,3,2,3,11,0,99,30,40,50];
            run_program(&mut input);
            input
        });

    let filename = "../day2/input.txt";
    let inputs: Vec<i32> = fs::read_to_string(filename)
        .expect("Something went wrong reading file.")
        .split(',')
        .filter_map(|n| n.parse::<i32>().ok())
        .collect();

    let mut part1 = inputs.clone();

    // Set state to 1202
    part1[1] = 12;
    part1[2] = 2;

    run_program(&mut part1);

    println!("The value at position 0 is {}", part1[0]);

    for n in 0..=99 {
        for v in 0..=99 {
            let mut part2 = inputs.clone();
            part2[1] = n;
            part2[2] = v;
            run_program(&mut part2);
            if part2[0] == 19690720 {
                println!("Noun = {}, Verb = {}, Result = {}",
                         n, v, 100 * n + v);
                break;
            }
        }
    }
}
