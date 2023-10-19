use std::fs;
use std::io::Read;

fn recurse(i: i32, j: i32, rows: i32, cols: i32, matrix: &mut Vec<Vec<usize>>) -> usize {
    if i < 0 || i >= rows || j < 0 || j >= cols || matrix[i as usize][j as usize] == 0 {
        return 0;
    }

    matrix[i as usize][j as usize] = 0;
    let ll = recurse(i - 1, j, rows, cols, matrix);
    let rr = recurse(i + 1, j, rows, cols, matrix);
    let tt = recurse(i, j - 1, rows, cols, matrix);
    let bb = recurse(i, j + 1, rows, cols, matrix);
    let tl = recurse(i - 1, j - 1, rows, cols, matrix);
    let tr = recurse(i + 1, j - 1, rows, cols, matrix);
    let bl = recurse(i - 1, j + 1, rows, cols, matrix);
    let br = recurse(i + 1, j + 1, rows, cols, matrix);

    1 + ll + rr + tt + bb + tl + tr + bl + br
}

fn connected_cell(matrix: &mut Vec<Vec<usize>>) -> i32 {
    let mut ans = 0;
    let rows = matrix.len();
    let cols = matrix[0].len();

    for i in 0..rows {
        for j in 0..cols {
            let d: usize = recurse(i as i32, j as i32, rows as i32, cols as i32, matrix);
            ans = ans.max(d);
        }
    }

    ans as i32
}

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("./src/input")?;
    let mut input: String = String::new();
    file.read_to_string(&mut input)?;
    let mut lines = input.lines();

    let rows = lines.next().unwrap().trim_end().parse::<usize>().unwrap();
    let cols = lines.next().unwrap().trim_end().parse::<usize>().unwrap();

    let mut matrix: Vec<Vec<usize>> = Vec::with_capacity(rows);

    for i in 0..rows {
        matrix.push(Vec::with_capacity(cols));

        matrix[i] = lines
            .next()
            .unwrap()
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
    }

    println!("{}", connected_cell(&mut matrix));

    Ok(())
}
