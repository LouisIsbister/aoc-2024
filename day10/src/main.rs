use std::collections::HashSet;

/// for every 0 in the input, perform a dfs from that point to find the number of reachable 9's
fn main() {
    let fstr = std::fs::read_to_string("input.txt").expect("Cannot read input!");
    let map = retrieve_map(&fstr);

    let ret1 = solution(&map, true);
    assert_eq!(744, ret1);

    let ret2 = solution(&map, false);
    assert_eq!(1651, ret2);
}

///
/// The only difference between part 1 & 2 is that part 2 tries to 
/// find all the permutations of paths to a specifc path end, whereas 
/// part 1 just wnats to find if there is a path. hence the only difference 
/// is that part 1 skips positions that have already been visited/seen,
/// where part 2 does not!
/// 
fn solution(map: &Vec<Vec<u8>>, is_part_1: bool) -> u64 {
    let mut counter: u64 = 0;
    for (x, y) in tailhead_starts(map) {
        dfs(
            &mut counter, 
            x as i64, 
            y as i64, 
            &mut HashSet::new(), 
            map, is_part_1
        );
    }
    counter
}

fn dfs(
    counter: &mut u64,
    cur_x: i64,
    cur_y: i64,
    seen: &mut HashSet<(i64, i64)>, 
    map: &Vec<Vec<u8>>,
    is_part_1: bool
) -> () {
    if map[cur_y as usize][cur_x as usize] == 9 {
        *counter += 1;
        return
    }

    let neighbours = get_neighbours(cur_x, cur_y, map);
    for (nx, ny) in neighbours {
        if seen.contains(&(nx, ny)) && is_part_1 {
            continue
        }
        
        if map[ny as usize][nx as usize] == map[cur_y as usize][cur_x as usize] + 1 {
            seen.insert((nx, ny));
            dfs(counter, nx, ny, seen, map, is_part_1)
        }
    }
}

fn get_neighbours(x: i64, y: i64, map: &Vec<Vec<u8>>) -> Vec<(i64, i64)> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .iter()
        .filter(|(x, y)| !out_of_bounds(*x, *y, map))
        .map(|(x, y)| (*x, *y))
        .collect::<Vec<(i64, i64)>>()
}

fn out_of_bounds(x: i64, y: i64, map: &Vec<Vec<u8>>) -> bool {
    x < 0 || x as usize >= map[0].len() || y < 0 || y as usize >= map.len() 
}

fn tailhead_starts(map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    map.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, val)| if *val == 0 { Some((x, y)) } else { None })
        })
        .collect()
}

fn retrieve_map(fstr: &String) -> Vec<Vec<u8>> {
    fstr.lines()
        .map(|line| 
            line.chars()
                .map(|c| c.to_string().parse::<u8>().expect("Not an int!"))
                .collect::<Vec<u8>>()
        )
        .collect::<Vec<Vec<u8>>>()
}
