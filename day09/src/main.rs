fn main() {
    let fstr = std::fs::read_to_string("input.txt").expect("Cannot read input!");
    let blocks = compute_blocks(&fstr);

    let ret1 = p1_solution(&mut blocks.clone());
    assert_eq!(6382875730645, ret1);

    let ret2 = p2_solution(&mut blocks.clone());
    assert_eq!(6420913943576, ret2);
}

fn p1_solution(blocks: &mut Vec<i64>) -> i64 {
    let mut lptr = 0;
    let mut rptr = blocks.len() - 1;
    while lptr <= rptr {
        lptr = iterate_to_next_free_space(lptr, &blocks);
        rptr = iterate_to_next_block(rptr, blocks);
        if lptr <= rptr {
            blocks.swap(lptr as usize, rptr as usize);
        }
    }

    blocks
        .iter()
        .enumerate()
        .map(|(idx, val)| 
            match val {
                -1 => 0,
                _ => idx as i64 * val
            }
        )
        .sum::<i64>()
}

fn p2_solution(blocks: &mut Vec<i64>) -> i64 {
    let mut rptr = blocks.len() as i64 - 1;

    while rptr >= 0 {
        rptr = iterate_to_next_block(rptr as usize, blocks) as i64;
        let id = blocks[rptr as usize];

        let mut len: i64 = 1;
        while rptr - len > 0 && blocks[(rptr - len) as usize] == id {
            len += 1;
        }

        swap_next_n_free_blocks(len, rptr, blocks);
        rptr -= len as i64;
    }

    blocks
        .iter()
        .enumerate()
        .map(|(idx, val)| 
            match val {
                -1 => 0,
                _ => idx as i64 * val
            }
        )
        .sum::<i64>()
}

fn swap_next_n_free_blocks(n: i64, rptr: i64, blocks: &mut Vec<i64>) {
    let mut counter: i64 = 0;
    for (idx, val) in blocks.clone().iter().enumerate() {
        if idx as i64 > rptr - n {
            return
        }

        counter = if *val == -1 { counter + 1 } else { 0 };
        if counter == n {
            for i in 0..(n as usize) {
                blocks.swap(rptr as usize - i, idx - n as usize + i + 1)
            }
        }
    }
} 

fn iterate_to_next_free_space(mut lptr: usize, blocks: &Vec<i64>) -> usize {
    while blocks[lptr] != -1 {
        lptr += 1;
    }
    lptr
}

fn iterate_to_next_block(mut rptr: usize, blocks: &Vec<i64>) -> usize {
    while blocks[rptr as usize] == -1 {
        rptr -= 1;
    }
    rptr
}

fn compute_blocks(fstr: &String) -> Vec<i64> {
    // vec of id, if id is None its a free space block 
    let mut ret: Vec<i64> = Vec::new();
    for (idx, ch) in fstr.chars().enumerate() {
        let id = (idx / 2) as i64;
        let number_of_blocks = ch.to_string().parse::<i64>().expect("Not an int!");
        for _ in 0..number_of_blocks {
            match idx % 2 == 0 {
                true => ret.push(id),
                false => ret.push(-1),
            }
        }
    }
    ret
}
