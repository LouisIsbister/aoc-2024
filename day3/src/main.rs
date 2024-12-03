use regex::Regex;

fn main() {
    let file = std::fs::read_to_string("input.txt").expect("Cannot read file!");
    let res = compute(&file);
    
    assert_eq!(59097164, res)
}

///
/// Part 1 and 2
/// Matches all the 'mul(?,?)'s, 'do()'s, and 'don't()'s, extracts the 
/// values and adds them if the previous conditional capture was a 'do()'
/// Finally returns the sum of the uncorrupted multiplications 
/// 
fn compute(file: &String) -> i64 {
    let pattern = Regex::new(r"(mul\([0-9][0-9]{0,2},[0-9][0-9]{0,2}\)|don't\(\)|do\(\))").expect("Invalid regex");
    let mut add = true;

    let mut res: i64 = 0;
    for capture in pattern.find_iter(file) {
        let cap = capture.as_str();
        match cap {
            "do()" => add = true, 
            "don't()" => add = false,
            _ if add => {
                let values: Vec<i64> = cap[4..cap.len() - 1].split(',') // remove leading 'mul(' and last ')' and split by comma
                    .map(|e| e.parse().expect("Not an int!"))
                    .collect();
                assert_eq!(2, values.len());
                res += values[0] * values[1];
            },
            _ => (),
        }
    }
    res
}
