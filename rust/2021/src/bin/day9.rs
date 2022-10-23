use commons::file_loading;
use std::collections::HashSet;


fn get_neighbouring_indexes(index: usize, width: i32) -> Vec<usize> {
    vec![
        index as i32 - width,
        index as i32 - 1,
        index as i32 + 1,
        index as i32 + width
    ].iter().filter(|&&n| {
        n > 0
    }).map(|&n| n as usize).filter(|&n| {
        !(n % width as usize == 0 && n == index + 1) && !(n % width as usize == width as usize - 1 && n == index - 1)
    }).collect()
}


fn calculate_risk(numbers: &Vec<u32>, width: i32) -> u32 {
    (0..numbers.len()).map(|i| {
        let current = numbers[i];
        let neighbours_indexes = get_neighbouring_indexes(i, width);
        if neighbours_indexes.iter().flat_map(|&n| numbers.get(n)).all(|n| n > &numbers[i]) {
            current + 1
        } else {
            0
        }
    }).sum()
}


fn find_basins(numbers: &Vec<u32>, width: i32) -> usize {
    let mut basins: Vec<HashSet<usize>> = Vec::new();

    for i in 0..numbers.len() {
        if basins.iter().any(|b| b.contains(&i)) {
            continue;
        }

        let mut next_basin: HashSet<usize> = HashSet::new();
        let mut indexes_to_check = vec![i];

        while indexes_to_check.len() != 0 {
            let next_to_check = indexes_to_check.pop().unwrap();
            if *numbers.get(next_to_check).unwrap_or(&9) != 9 {
                next_basin.insert(next_to_check);
                let neighbours = get_neighbouring_indexes(next_to_check, width);
                indexes_to_check.extend(neighbours.iter().filter(|&i| !next_basin.contains(i)).collect::<Vec<&usize>>());
            }
        }
        basins.push(next_basin);
    }

    let mut sizes = basins.iter().map(|b| b.len()).collect::<Vec<usize>>();
    sizes.sort_by(|a, b| b.cmp(a));
    sizes.iter().take(3).product()
}


fn main() {
    let content = file_loading::read_file_to_string(2021, 9).unwrap();
    let rows = content.split("\n");
    let width = rows.clone().count() as i32;

    let numbers = rows.flat_map(|s| {
        s.chars().map(|c| c.to_digit(10).unwrap())
    }).collect::<Vec<u32>>();

    let risk = calculate_risk(&numbers, width);
    println!("{}", risk);

    let basins = find_basins(&numbers, width);
    println!("{}", basins);
}
