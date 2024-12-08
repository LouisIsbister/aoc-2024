use regex::Regex;
use std::collections::HashSet;

fn main() {
    let fstr = std::fs::read_to_string("input.txt").expect("Could not open input file!");
    let tests = parse_input(&fstr);

    let ret1 = solution(&tests, false);
    assert_eq!(1708857123053, ret1);

    let ret2 = solution(&tests, true);
    assert_eq!(189207836795655, ret2)
}

fn solution(tests: &Vec<(u64, Vec<u64>)>, is_part2: bool) -> u64 {
    let mut ret: u64 = 0;
    for (target, operands) in tests {
        let mut current_layer: HashSet<u64> = HashSet::new();
        current_layer.insert(operands[0]);

        let mut ptr: usize = 1;
        while ptr < operands.len() {
            let mut next_layer = HashSet::new();
            for &val in &current_layer {
                next_layer.insert(operands[ptr] * val);
                next_layer.insert(operands[ptr] + val);
                if is_part2 {
                    next_layer.insert((val.to_string() + &operands[ptr].to_string()).parse::<u64>().expect("Not an int!"));
                }
            }
            current_layer = next_layer;
            ptr += 1;
        }
        if current_layer.contains(target) {
            ret += *target;
        }
    }
    ret
}

fn parse_input(fstr: &String) -> Vec<(u64, Vec<u64>)> {
    let pat = Regex::new(r"(:\s|\s)").expect("Invalid regex!");
    fstr.lines()
        .map(|line| {
            let vals = pat.split(line)
                .map(|e| e.parse::<u64>().expect("Not an int!"))
                .collect::<Vec<_>>();
            (vals[0], vals[1..].to_vec())
        })
        .collect()
}