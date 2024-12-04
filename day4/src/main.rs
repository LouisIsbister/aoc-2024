mod part1;
use part1::p1_solution;

mod part2;
use part2::p2_solution;

fn main() {
    let file = std::fs::read_to_string("input.txt").expect("Cannot read file!");
    let p1_res = p1_solution(&file);
    assert_eq!(2447, p1_res);

    let p2_res = p2_solution(&file);
    assert_eq!(1868, p2_res);
}
