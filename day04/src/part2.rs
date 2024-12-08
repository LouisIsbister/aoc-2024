use crate::part1::file_to_matrix;

pub fn p2_solution(file: &String) -> u64 {
    let lines = file_to_matrix(file);
    let mut counter = 0;
    for y in 1..lines.len() - 1 {
        for x in 1..lines[y].len() - 1 {
            if lines[y][x].eq("A") {
                x_at_point(x, y, &lines, &mut counter);
            }
        }
    }
    counter
}

fn x_at_point(x: usize, y: usize, mat: &Vec<Vec<String>>, counter: &mut u64) -> () {
    let top_l = &mat[y - 1][x - 1];
    let top_r = &mat[y - 1][x + 1];
    let bot_l = &mat[y + 1][x - 1];
    let bot_r = &mat[y + 1][x + 1];

    if ((top_l == "M" && bot_r == "S") || (top_l == "S" && bot_r == "M"))
        && ((top_r == "M" && bot_l == "S") || (top_r == "S" && bot_l == "M"))
    {
        *counter += 1;
    }
}
