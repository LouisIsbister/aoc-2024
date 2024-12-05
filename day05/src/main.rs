use std::collections::HashMap;

fn main() {
    let file = std::fs::read_to_string("input.txt").expect("Cannot read input!");
    let (ordering, tasks) = extract_instructions(&file);
    
    let ret1 = p1_solution(&ordering, &tasks);
    assert_eq!(6505, ret1);

    let ret2 = p2_solution(&ordering, &tasks);
    assert_eq!(6897, ret2);
}

fn p1_solution(ordering: &HashMap<i64, Vec<i64>>, tasks: &Vec<Vec<i64>>) -> i64 {
    tasks.iter()
        .map(|task| 
            if is_valid_task(task, ordering) { 
                task[task.len() / 2] 
            } else { 
                0 
            })
        .sum::<i64>()
}

fn p2_solution(ordering: &HashMap<i64, Vec<i64>>, tasks: &Vec<Vec<i64>>) -> i64 {
    tasks.iter()
        .cloned()
        .filter(|task| !is_valid_task(&task, &ordering))  // get all the invalid tasks
        .map(|task| fix_task(task.clone(), &ordering))   // fix all the invalid tasks
        .map(|task| task[task.len() / 2])
        .sum::<i64>()
}

fn fix_task(mut task: Vec<i64>, ordering: &HashMap<i64, Vec<i64>>) -> Vec<i64> {
    while !is_valid_task(&task, ordering){
        for i in 1..task.len() {
            let prev_key = task[i - 1];
            let cur_value = task[i];

            let order  = ordering.get(&prev_key).expect("Not in the map!");
            if !order.contains(&cur_value) {
                task.swap(i - 1, i);
            }
        }
    }
    task
}

///
/// util
/// 

fn is_valid_task(task: &Vec<i64>, ordering: &HashMap<i64, Vec<i64>>) -> bool {
    for i in 1..task.len() {
        let prev_key = task[i - 1];
        let cur_value = task[i];

        let order  = ordering.get(&prev_key).expect("Not in the map!");
        if !order.contains(&cur_value) {
            return false
        }
    }
    true
}

///
/// Extraction the order rules and tasks from the input file
/// 

fn extract_instructions(file: &String) -> (HashMap<i64, Vec<i64>>, Vec<Vec<i64>>) {
    let mut lines = file.lines();   
    
    // parse the printing orders
    let mut ordering: HashMap<i64, Vec<i64>> = HashMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() { 
            break
        }
        parse_order(&line, &mut ordering);
    }

    // parse the tasks
    let mut tasks: Vec<Vec<i64>> = Vec::new();
    while let Some(line) = lines.next() {
        parse_task(&line, &mut tasks)
    }

    (ordering, tasks)
}

fn parse_order(line: &str, ordering: &mut HashMap<i64, Vec<i64>>) {
    let mut split = line.split("|");
    assert_eq!(2, split.clone().count());
    
    let k: i64 = split.next().and_then(|e| e.parse().ok()).expect("Not an int!");
    let v: i64 = split.next().and_then(|e| e.parse().ok()).expect("Not an int!");

    ordering.entry(k).or_insert_with(Vec::new).push(v)
}

fn parse_task(line: &str, tasks: &mut Vec<Vec<i64>>) {
    let split = line.split(",");
    tasks.push(split.map(|e| e.parse().expect("Not an int!")).collect());
}