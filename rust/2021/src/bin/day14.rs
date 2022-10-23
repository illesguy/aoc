use commons::file_loading;
use std::collections::HashMap;


fn apply_rules(template: HashMap<String, u64>, insertions: &Vec<(&str, &str)>) -> HashMap<String, u64> {
    let mut new_template = template.clone();

    for &(k, v) in insertions.iter() {
        let new1 = format!("{}{}", k.chars().nth(0).unwrap(), v);
        let new2 = format!("{}{}", v, k.chars().nth(1).unwrap());
        let old_count = *template.get(k).unwrap_or(&0);

        *new_template.entry(new1).or_insert(0) += old_count;
        *new_template.entry(new2).or_insert(0) += old_count;
        *new_template.entry(k.to_string()).or_insert(0) -= old_count;
    }
    new_template
}


fn find_difference_between(template: &HashMap<String, u64>, first_char: char, last_char: char) -> u64 {
    let mut char_counts: HashMap<char, u64> = HashMap::new();
    char_counts.insert(first_char, 1);
    char_counts.insert(last_char, 1);

    for (k, v) in template.iter() {
        for c in k.chars() {
            let e = char_counts.entry(c).or_insert(0);
            *e += v;
        }
    }

    let counts_vec = char_counts.into_iter().map(|(_, v)| v).collect::<Vec<u64>>();
    (counts_vec.iter().max().unwrap() - counts_vec.iter().min().unwrap()) / 2
}


pub fn main() {
    let content = file_loading::read_file_to_string(2021, 14).unwrap();
    let (head, tail) = content.split_once("\n\n").unwrap();

    let mut template: HashMap<String, u64> = HashMap::new();
    let first_char = head.chars().next().unwrap();
    let last_char = head.chars().last().unwrap();
    for i in 1..head.len() {
        let c = template.entry(head[i-1..=i].to_string()).or_insert(0);
        *c += 1
    }
    let insertions = tail.split("\n").map(|i| i.split_once(" -> ").unwrap()).collect::<Vec<(&str, &str)>>();

    for _ in 0..10 {
        template = apply_rules(template, &insertions);
    }

    let diff10 = find_difference_between(&template, first_char, last_char);
    println!("{}", diff10);

    for _ in 0..30 {
        template = apply_rules(template, &insertions);
    }

    let diff40 = find_difference_between(&template, first_char, last_char);
    println!("{}", diff40);
}