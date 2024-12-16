use std::collections::HashMap;

fn main() {
    let fstr = std::fs::read_to_string("input.txt").expect("Cannot read input!");
    let line = extract_data(&fstr);

    let ret1 = solution(line.clone(), 25);
    assert_eq!(185205, ret1);

    let ret2 = solution(line.clone(), 75);
    assert_eq!(221280540398419, ret2);
}

fn solution(line: Vec<u64>, iterations: usize) -> usize {
    let mut ret = 0;
    let mut cache: HashMap<(u64, usize), u64> = HashMap::new();

    for stone in line {
        ret += compute_stone_count(stone, iterations, &mut cache);
    }
    ret
}

fn compute_stone_count(stone: u64, iterations: usize, cache: &mut HashMap<(u64, usize), u64>) -> usize {
    if iterations == 0 {
        return 1
    }

    if let Some(count) = cache.get(&(stone, iterations)) {
        return *count as usize
    }

    let vals = next(stone);
    let mut res = 0; 
    for val in vals {
        res += compute_stone_count(val, iterations - 1, cache);
    }
    cache.insert((stone, iterations), res as u64);
    res
}

fn next(stone: u64) -> Vec<u64> {
    let val_str = stone.to_string();
    if stone == 0 {
        vec![1]
    } else if val_str.len() % 2 == 0 {
        let vs = val_str.split_at(val_str.len() / 2);
        vec![
            vs.0.parse::<u64>().expect("Not an int!"),
            vs.1.parse::<u64>().expect("Not an int!"),
        ]
    } else {
        vec![stone * 2024]
    }
}

fn extract_data(fstr: &String) -> Vec<u64> {
    fstr.split_whitespace()
        .map(|val| val.parse::<u64>().expect("Not an int!"))
        .collect()
}
