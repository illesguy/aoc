use commons::file_loading;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::iter::FromIterator;


fn count_obvious_digits(parsed_input: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    parsed_input.iter().map(|(_, out)| {
        out.iter().filter(|s| {
            let str_len = s.len();
            str_len == 2 || str_len == 3 || str_len == 4 || str_len == 7
        }).count()
    }).sum()
}


fn decode_line(inputs: &Vec<String>, outputs: &Vec<String>) -> i32 {
    let unique_inputs = inputs.iter().map(|s| {
        s.chars().collect::<HashSet<char>>()
    }).collect::<Vec<HashSet<char>>>();

    let mut mappings: HashMap<i32, &HashSet<char>> = HashMap::new();
    mappings.insert(1, unique_inputs.iter().find(|c| c.len() == 2).unwrap());
    mappings.insert(4, unique_inputs.iter().find(|c| c.len() == 4).unwrap());
    mappings.insert(7, unique_inputs.iter().find(|c| c.len() == 3).unwrap());
    mappings.insert(8, unique_inputs.iter().find(|c| c.len() == 7).unwrap());
    
    mappings.insert(9, unique_inputs.iter().find(|&c| {
        c.len() == 6 &&
        c.difference(
            &mappings.get(&4).unwrap().union(
                mappings.get(&7).unwrap()
            ).cloned().collect::<HashSet<char>>()
        ).count() == 1
    }).unwrap());

    mappings.insert(0, unique_inputs.iter().find(|&c| {
        c.len() == 6 && c.difference(&mappings.get(&9).unwrap()).count() == 1 &&
        c.intersection(&mappings.get(&1).unwrap()).count() == 2
    }).unwrap());

    mappings.insert(6, unique_inputs.iter().find(|&c| {
        c.len() == 6 && c != *mappings.get(&9).unwrap() && c != *mappings.get(&0).unwrap()
    }).unwrap());

    mappings.insert(3, unique_inputs.iter().find(|&c| {    
        c.len() == 5 && c.intersection(mappings.get(&1).unwrap()).count() == 2
    }).unwrap());
    
    mappings.insert(5, unique_inputs.iter().find(|&c| {
        c.len() == 5 && c != *mappings.get(&3).unwrap() && c.intersection(mappings.get(&4).unwrap()).count() == 3
    }).unwrap());
    
    mappings.insert(2, unique_inputs.iter().find(|&c| {
        c.len() == 5 && c != *mappings.get(&5).unwrap() && c != *mappings.get(&3).unwrap()
    }).unwrap());

    let done_mappings = mappings.iter().map(|(&num, &code)| {
        let mut chars = code.iter().cloned().collect::<Vec<char>>();
        chars.sort_by(|a, b| a.cmp(b));
        (String::from_iter(chars), num)
    }).collect::<HashMap<String, i32>>();
    
    let mut res = 0;
    for i in 0..4 {
        let mut chars = outputs[i].chars().collect::<Vec<char>>();
        chars.sort_by(|a, b| a.cmp(b));
        let key = String::from_iter(chars);
        res += done_mappings.get(&key).unwrap() * i32::pow(10, 3 - i as u32);
    }
    res
}



fn main() {
    let content = file_loading::read_file_to_string(2021, 8).unwrap();
    let parsed = content.split("\n").map(|line| {
        let (inp, outp) = line.split_once(" | ").unwrap();
        let inp_vec = inp.to_string().split_whitespace().map(|s| s.to_string()).collect();
        let outp_vec = outp.to_string().split_whitespace().map(|s| s.to_string()).collect();
        (inp_vec, outp_vec)
    }).collect::<Vec<(Vec<String>, Vec<String>)>>();

    let digit_count = count_obvious_digits(&parsed);
    println!("{}", digit_count);

    let full_decoded_output: i32 = parsed.iter().map(|(inp, outp)| {
        decode_line(inp, outp)
    }).sum();
    println!("{}", full_decoded_output);
}
