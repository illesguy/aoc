use commons::file_loading;


fn calculate_step_cost(steps: i32) -> i32 {
    steps * (steps + 1) / 2
}


fn main() {
    let content = file_loading::read_file_to_string(2021, 7).unwrap();
    let mut positions = content.split(",").map(|p| p.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mid = positions.len() / 2;
    
    positions.sort();
    let best_pos = positions[mid];
    let min_move: i32 = positions.iter().map(|p| (p - best_pos).abs()).sum();
    println!("{}", min_move);

    let sum: i32 = Iterator::sum(positions.iter());
    let best_pos2 = (f64::from(sum) / (positions.len() as f64)) as i32;
    let min_mov2: i32 = positions.iter().map(|p| calculate_step_cost((p - best_pos2).abs())).sum();
    println!("{}", min_mov2);
}
