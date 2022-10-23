use commons::file_loading;
use std::collections::HashMap;
use std::cmp::min;


fn find_fastest_route_from(grid: &Vec<Vec<u32>>, row: usize, col: usize, mappings: &mut HashMap<(usize, usize), u32>) -> u32 {
    if mappings.contains_key(&(row, col)) {
        *mappings.get(&(row, col)).unwrap()
    } else {
        let current = grid[row][col];
        if row == grid.len() - 1 && col == grid[row].len() - 1 {
            let res = current - grid[0][0];
            mappings.insert((row, col), res);
            res
        } else if row == grid.len() - 1 {
            let res = current + find_fastest_route_from(grid, row, col + 1, mappings);
            mappings.insert((row, col), res);
            res
        } else if col == grid[row].len() - 1 {
            let res = current + find_fastest_route_from(grid, row + 1, col, mappings);
            mappings.insert((row, col), res);
            res
        } else {
            let res = current + min(
            find_fastest_route_from(grid, row + 1, col, mappings),
            find_fastest_route_from(grid, row, col + 1, mappings)
            );
            mappings.insert((row, col), res);
            res
        }
    }
}


fn generate_new_grid(grid: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut new_grid = Vec::new();
    for ra in 0..5 {
        for r in grid {
            let mut new_row: Vec<u32> = Vec::new();
            for ca in 0..5 {
                for &e in r {
                    new_row.push(wrap_number(e + ra + ca));
                }
            }
            new_grid.push(new_row);
        }
    }
    new_grid
}


fn wrap_number(n: u32) -> u32 {
    ((n - 1) % 9) + 1
}


pub fn main() {
    let content = file_loading::read_file_to_string(2021, 15).unwrap();
    let grid = content.split("\n").map(|row| {
        row.chars().map(|c| c.to_digit(10).unwrap()).collect()
    }).collect::<Vec<Vec<u32>>>();

    let min_path = find_fastest_route_from(&grid, 0, 0, &mut HashMap::new());
    println!("{}", min_path);
    
    let new_grid = generate_new_grid(&grid);
    let min_path_larger = find_fastest_route_from(&new_grid, 0, 0, &mut HashMap::new());
    println!("{}", min_path_larger);
}
