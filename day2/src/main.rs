fn main() {
    let file = std::fs::read_to_string("input.txt").expect("Cannot read file!");
    let levels: Vec<Vec<i64>> = parse_levels(&file);

    let safe_count = safe_level_count(levels);
    assert_eq!(692, safe_count);
}

fn safe_level_count(levels: Vec<Vec<i64>>) -> u64 {
    levels.iter()
        .filter(|level| is_decr(*level) || is_incr(*level) || check_dampening(*level))
        .count() as u64
}

///
/// Part 2
/// Check whether removing an element makes the level valid
/// 
fn check_dampening(lev: &Vec<i64>) -> bool {
    let mut copy = lev.clone();
    for i in 0..lev.len() {
        let rm: i64 = copy.remove(i);
        if is_decr(&copy) || is_incr(&copy) {
            return true;
        }
        copy.insert(i, rm);
    }
    false
}

///
/// Part 1
/// 

fn is_decr(lev: &Vec<i64>) -> bool {
    check_incr_decr(lev, |a, b| a < b)
}

fn is_incr(lev: &Vec<i64>) -> bool {
    check_incr_decr(lev, |a, b| a > b)
}

fn check_incr_decr(lev: &Vec<i64>, func: impl Fn(i64, i64) -> bool) -> bool {
    let mut cur = lev.get(0).unwrap();
    for i in 1..lev.len() {
        let next = lev.get(i).unwrap();
        let diff = (cur - next).abs();
        if !func(*cur, *next) || diff > 3 || diff < 1 {
            return false;
        }
        cur = next;
    }
    true
}

///
/// Load the levels from input file
/// 
fn parse_levels(file: &String) -> Vec<Vec<i64>> {
    file.lines()
        .map(|line| line.split(" ").map(|e| e.parse().unwrap()).collect())
        .collect()
}
