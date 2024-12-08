pub fn file_to_matrix(file: &String) -> Vec<Vec<String>> {
    file.lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<_>>())
        .collect()
}

pub fn p1_solution(file: &String) -> u64 {
    let lines = file_to_matrix(file);
    let directions: Vec<(i32, i32)> = vec![
        (1, -1), (1, 0), (1, 1),
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
    ];

    let mut counter = 0;
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x].eq("X") {
                check_xmas_at_point(x as i32, y as i32, &lines, &directions, &mut counter);
            }
        }
    }
    counter
}

fn check_xmas_at_point(
    x: i32,
    y: i32,
    mat: &Vec<Vec<String>>,
    directions: &Vec<(i32, i32)>,
    counter: &mut u64,
) {
    for (xincr, yincr) in directions {
        if !check_eq(x, y, "X", mat) {
            continue;
        }
        if !check_eq(x + xincr, y + yincr, "M", mat) {
            continue;
        }
        if !check_eq(x + 2 * xincr, y + 2 * yincr, "A", mat) {
            continue;
        }
        if check_eq(x + 3 * xincr, y + 3 * yincr, "S", mat) {
            *counter += 1
        } // found an XMAS
    }
}

fn check_eq(x: i32, y: i32, check: &str, mat: &Vec<Vec<String>>) -> bool {
    if y >= 0 && (y as usize) < mat.len() && x >= 0 && (x as usize) < mat[0].len() {
        check.eq(&mat[y as usize][x as usize])
    } else {
        false
    }
}
