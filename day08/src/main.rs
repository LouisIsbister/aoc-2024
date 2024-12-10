use std::collections::{HashSet, HashMap};

fn main() {
    let fstr = std::fs::read_to_string("input.txt").expect("Cannot read input!");
    let (matrix, coords) = retrieve_data(&fstr);
    
    let ret1 = p1_solution(&matrix, &coords);
    assert_eq!(285, ret1);

    let ret2 = p2_solution(&matrix, &coords);
    assert_eq!(944, ret2)
}

fn p1_solution(matrix: &Vec<Vec<String>>, coords: &HashMap<String, Vec<(i64, i64)>>) -> u64 {
    let mut ret: u64 = 0;
    let mut seen: HashSet<(i64, i64)> = HashSet::new();

    for (_, values) in coords {
        for (idx, c1) in values[0..values.len() - 1].iter().enumerate() {
            for c2 in &values[idx + 1..] {
                if c1 == c2 {
                    continue
                }
            
                let xshift = c2.0 - c1.0;
                let yshift = c2.1 - c1.1;
            
                let c1_sh = (c1.0 - xshift, c1.1 - yshift);
                if check_bounds(&matrix, &c1_sh) && !seen.contains(&c1_sh) {
                    ret += 1;
                    seen.insert(c1_sh);
                }
            
                let c2_sh = (c2.0 + xshift, c2.1 + yshift);
                if check_bounds(&matrix, &c2_sh) && !seen.contains(&c2_sh) {
                    ret += 1;
                    seen.insert(c2_sh);
                }
            }
        }
    }
    ret
}

fn p2_solution(matrix: &Vec<Vec<String>>, coords: &HashMap<String, Vec<(i64, i64)>>) -> u64 {
    let mut ret: u64 = 0;
    let mut seen: HashSet<(i64, i64)> = HashSet::new();

    for (_, values) in coords {
        for (idx, c1) in values[0..values.len() - 1].iter().enumerate() {
            for c2 in &values[idx + 1..] {
                if c1 == c2 {
                    continue
                }
            
                let xshift = c2.0 - c1.0;
                let yshift = c2.1 - c1.1;
                ret += compute_anti_nodes_p2(matrix, &mut seen, *c1, xshift * -1, yshift * -1);
                ret += compute_anti_nodes_p2(matrix, &mut seen, *c2, xshift, yshift);
            }
        }
    }
    ret
}

fn compute_anti_nodes_p2(
    matrix: &Vec<Vec<String>>, 
    seen: &mut HashSet<(i64, i64)>, 
    mut c: (i64, i64), 
    xshift: i64, 
    yshift: i64
) -> u64 {
    let mut ret = 0;
    while check_bounds(&matrix, &c) {
        if !seen.contains(&c) {
            ret += 1
        }
        seen.insert(c);
        c = (c.0 + xshift, c.1 + yshift)
    }
    ret
}

fn check_bounds(matrix: &Vec<Vec<String>>, c: &(i64, i64)) -> bool {
    c.0 >= 0 && c.1 >= 0 && c.0 < matrix[0].len() as i64 && c.1 < matrix.len() as i64
}

fn retrieve_data(fstr: &String) -> (Vec<Vec<String>>, HashMap<String, Vec<(i64, i64)>>) {
    let mut ret: HashMap<String, Vec<(i64, i64)>> = HashMap::new();

    let matrix = fstr.lines()
        .map(|line| line.chars().map(|ch| ch.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    for (y, line) in matrix.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            match val.as_str() {
                "." => (),
                _ => {
                    
                    if let Some(values) = ret.get(val) {
                        let mut tmp_vals = values.clone();
                        tmp_vals.push((x as i64, y as i64));
                        ret.insert(val.clone(), tmp_vals);
                    } else {
                        ret.insert(val.clone(), vec![(x as i64, y as i64)]);
                    }
                }
            }
        }
    }
    (matrix, ret)
}


