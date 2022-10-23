use commons::file_loading;
use std::collections::{VecDeque, HashSet};


fn get_neighbours(index: usize, grid_width: usize) -> Vec<usize> {
    let get_single_neighbour = |vertical: i32, horizontal: i32| -> Option<usize> {
        if vertical == 0 && horizontal == 0 {
            None
        } else if horizontal == -1 && index % grid_width == 0 {
            None
        } else if horizontal == 1 && index % grid_width == grid_width - 1 {
            None
        } else {
            let neighbour = index as i32 + horizontal + grid_width as i32 * vertical;
            if neighbour < 0 || neighbour >= grid_width as i32 * grid_width as i32 {
                None
            } else {
                Some(neighbour as usize)
            }
        }

    };

    (-1..=1).flat_map(|v: i32| {
        (-1..=1).flat_map(move |h: i32| get_single_neighbour(v, h))
    }).collect::<Vec<usize>>()
}


fn single_step(energy_levels: &mut Vec<u32>, grid_width: usize) -> u32 {
    let mut flashes = 0;
    let mut indexes_to_flash: VecDeque<usize> = VecDeque::new();
    let mut flashed_this_step: HashSet<usize> = HashSet::new();
    for i in 0..energy_levels.len() {
        energy_levels[i] += 1;
        if energy_levels[i] > 9 {
            indexes_to_flash.push_back(i);
            flashed_this_step.insert(i);
        }
    }

    while indexes_to_flash.len() != 0 {
        let next_to_flash = indexes_to_flash.pop_front().unwrap();
        flashes += 1;
        for n in get_neighbours(next_to_flash, grid_width) {
            energy_levels[n] += 1;
            if energy_levels[n] > 9 && !flashed_this_step.contains(&n) {
                flashed_this_step.insert(n);
                indexes_to_flash.push_back(n);
            }
        }
    }

    for i in 0..energy_levels.len() {
        if energy_levels[i] > 9 {
            energy_levels[i] = 0;
        }
    }
    flashes
}


fn energy_step(mut energy_levels: Vec<u32>, grid_width: usize, steps: u32) -> u32 {
    let mut flashes = 0;
    for _ in 0..steps {
        flashes += single_step(&mut energy_levels, grid_width);
    }
    flashes
}


fn find_step_with_all(mut energy_levels: Vec<u32>, grid_width: usize) -> u32 {
    let mut steps = 1;
    while single_step(&mut energy_levels, grid_width) != grid_width as u32 * grid_width as u32 {
        steps += 1;
    }
    steps
}


pub fn main() {
    let content = file_loading::read_file_to_string(2021, 11).unwrap();
    let lines = content.split("\n");
    let grid_width = lines.clone().count();
    let energy_levels = lines.flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap())).collect::<Vec<u32>>();

    let total_flashes = energy_step(energy_levels.clone(), grid_width, 100);
    println!("{}", total_flashes);

    let step_with_all = find_step_with_all(energy_levels.clone(), grid_width);
    println!("{}", step_with_all);
}
