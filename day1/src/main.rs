fn main() {
    let file = std::fs::read_to_string("input.txt").expect("Cannot read file!");
    let (l1, l2) = compute_lists(&file);

    let res1 = distance(l1.clone(), l2.clone());
    assert_eq!(2086478, res1);

    let res2 = similarity_score(l1.clone(), l2.clone());
    assert_eq!(24941624, res2);
}

///
/// Task 2
/// 
fn distance(mut l1: Vec<i64>, mut l2: Vec<i64>) -> i64 {
    l1.sort_by(|a, b| a.cmp(b));
    l2.sort_by(|a, b| a.cmp(b));
    l1.iter().zip(l2.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
}

///
/// Task 1
/// 
fn similarity_score(l1: Vec<i64>, l2: Vec<i64>) -> i64 {
    l1.iter().fold(
        0i64, 
        |acc, x| acc + x * l2.iter().filter(|y| *x == **y).count() as i64
    )
}

///
/// Returns 2 vectors of integers retrieved from the 
/// input file
/// 
fn compute_lists(file: &String) -> (Vec<i64>, Vec<i64>) {
    let mut l1: Vec<i64> = Vec::new();
    let mut l2: Vec<i64> = Vec::new();
    
    for line in file.lines() {
        let res: Vec<i64> = line.split("   ")
                .map(|val| val.parse().expect("Not an int!"))
                .collect();
        l1.push(*res.get(0).expect("Cannot get first element!"));
        l2.push(*res.get(1).expect("Cannot get second element!"));
    }
    (l1, l2)
}
