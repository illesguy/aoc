use commons::file_loading;
use std::collections::HashMap;


fn map_count_after(mut fish: HashMap<i64, i64>, days_to_pass: i64) -> i64 {
    if days_to_pass == 0 {
        fish.iter().map(|(_, v)| v).sum()
    } else {
        let zeroes = fish.get(&0).unwrap_or(&0).clone();
        for i in 0..8 {
            let next = fish.get(&(i + 1)).unwrap_or(&0).clone();
            let count = fish.entry(i).or_insert(0);
            *count = next;
        }
        let c6 = fish.entry(6).or_insert(0);
        *c6 += zeroes;

        let c8 = fish.entry(8).or_insert(0);
        *c8 = zeroes;

        map_count_after(fish, days_to_pass - 1)
    }
}


fn main() {
    let content = file_loading::read_file_to_string(2021, 6).unwrap();
    let fish_ages = content.split(",").map(|f| f.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut fish: HashMap<i64, i64> = HashMap::new();
    for f in fish_ages {
        let count = fish.entry(f).or_insert(0);
        *count += 1;
    }

    println!("{}", map_count_after(fish.clone(), 80));
    println!("{}", map_count_after(fish.clone(), 256));
}
