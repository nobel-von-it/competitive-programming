pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let ln = grid.len();
    let mut max_local = vec![vec![0; ln - 2]; ln - 2];

    fn find_max(grid: &[Vec<i32>], x: usize, y: usize) -> i32 {
        grid.iter().skip(x).take(3).fold(0, |ac, row| {
            let max_row = row.iter().skip(y).take(3).max().unwrap();
            ac.max(*max_row)
        })
    }

    max_local
        .iter_mut()
        .enumerate()
        .take(ln - 2)
        .for_each(|(i, row)| {
            row.iter_mut()
                .enumerate()
                .take(ln - 2)
                .for_each(|(j, cell)| {
                    *cell = find_max(&grid, i, j);
                });
        });
    max_local
}
