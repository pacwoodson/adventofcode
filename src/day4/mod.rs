pub fn solve(input: &String) -> (String, String) {
    let original_grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            line.bytes()
                .into_iter()
                .map(|c| if c == b'@' { true } else { false })
                .collect()
        })
        .collect();

    let (_, original_n_movable, _) = get_movable_grid(&original_grid);

    let mut current_grid = original_grid.clone();
    let mut removable_rolls = 0;
    loop {
        let (_, n_movable, grid_post_remove) = get_movable_grid(&current_grid);
        current_grid = grid_post_remove;
        if n_movable == 0 {break;}
        removable_rolls += n_movable;
    }

    // println!("\nGrid:");
    // print_grid(&grid);
    // println!("\nMovables:");
    // print_grid(&movables);
    // println!("\nNumber movables: {}", n_movable);

    (original_n_movable.to_string(), removable_rolls.to_string())
}

fn get_movable_grid(grid: &Vec<Vec<bool>>) -> (Vec<Vec<bool>>, usize, Vec<Vec<bool>>) {
    let movables: Vec<Vec<bool>> = grid
        .iter()
        .enumerate()
        .map(|(y, row)| 
            row.iter()
                .enumerate()
                .map(|(x, _)| grid[y][x] && is_moveable(&grid, x, y))
                .collect()
        ).collect();

    let grid_post_remove =  grid
        .iter()
        .enumerate()
        .map(|(y, row)| 
            row.iter()
                .enumerate()
                .map(|(x, _)| if movables[y][x] {false} else {grid[y][x]})
                .collect()
        ).collect();

    let n_movable: usize = movables
        .iter()
        .map(|row| {
            row.iter()
                .map(|block| if *block { 1 as usize } else { 0 as usize })
                .sum::<usize>()
        })
        .sum();

    (movables, n_movable, grid_post_remove)
}

fn is_moveable(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    let mut neighbours = 0;
    for y_n in (y.saturating_sub(1))..grid.len().min(y + 2) {
        for x_n in (x.saturating_sub(1))..grid[y].len().min(x + 2) {
            if x_n == x && y_n == y {
                continue;
            }
            if grid[y_n][x_n] {
                neighbours += 1;
            }
        }
    }
    neighbours < 4
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<bool>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", if grid[y][x] { '@' } else { '.' });
        }
        print!("\n");
    }
}
