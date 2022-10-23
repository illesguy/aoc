use commons::file_loading;
use std::collections::VecDeque;


fn get_char_error_score(error_char: char) -> i32 {
    match error_char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unknown character {}", error_char)
    }
}


fn get_char_completion_score(c: char) -> i64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("Unknown character {}", c)
    }
}


fn char_is_opening(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}


fn get_char_pair(error_char: char) -> char {
    match error_char {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Unknown character {}", error_char)
    }
}


fn find_line_error_score(line: &str) -> i32 {
    let mut stack: VecDeque<char> = VecDeque::new();

    for c in line.chars() {
        if char_is_opening(c) {
            stack.push_front(c)
        } else {
            let top = stack.pop_front().unwrap();
            if c != get_char_pair(top) {
                return get_char_error_score(c)
            }
        }
    }
    
    0
}


fn find_line_remainders(line: &str) -> VecDeque<char> {
    let mut stack: VecDeque<char> = VecDeque::new();

    for c in line.chars() {
        if char_is_opening(c) {
            stack.push_front(c)
        } else {
            let top = stack.pop_front().unwrap();
            if c != get_char_pair(top) {
                return VecDeque::new()
            }
        }
    }
    
    stack
}


fn find_line_completion_score(line_remainders: &VecDeque<char>) -> Option<i64> {
    if line_remainders.is_empty() {
        None
    } else {
        let mut score: i64 = 0;
        for &c in line_remainders {
            score = score * 5 + get_char_completion_score(c)
        }
        Some(score)
    }
}


fn main() {
    let content = file_loading::read_file_to_string(2021, 10).unwrap();
    let lines = content.split("\n").collect::<Vec<&str>>();

    let error_score: i32 = lines.iter().map(|s| find_line_error_score(s)).sum();
    println!("{}", error_score);

    let mut completion_scores: Vec<i64> = lines.iter().flat_map(|s| find_line_completion_score(&find_line_remainders(s))).collect::<Vec<i64>>();
    completion_scores.sort_by(|a, b| a.cmp(b));
    let completion_score = completion_scores[completion_scores.len() / 2];
    println!("{}", completion_score);
}
