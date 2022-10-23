use std::collections::{HashMap, HashSet};
use commons::file_loading;


struct Graph {
    layout: HashMap<String, HashSet<String>>
}

impl Graph {
    fn from_string(s: String) -> Graph {
        let mut layout: HashMap<String, HashSet<String>> = HashMap::new();
        for path in s.split("\n") {
            let (start, end) = path.split_once("-").unwrap();
            let start_entry = layout.entry(start.to_string()).or_insert(HashSet::new());
            start_entry.insert(end.to_string());
            let end_entry = layout.entry(end.to_string()).or_insert(HashSet::new());
            end_entry.insert(start.to_string());
        }
        Graph {
            layout
        }
    }

    fn find_all_paths(&self, allow_single_return: bool) -> Vec<Vec<String>> {
        fn find_paths_from(layout: &HashMap<String, HashSet<String>>, current: String, mut path_so_far: Vec<String>, mut caves_visited: HashSet<String>, can_revisit: bool) -> Option<Vec<Vec<String>>> {
            // println!("====================");
            // println!("layout: {:?}", layout);
            // println!("current: {}", current);
            // println!("path so far: {:?}", path_so_far);
            // println!("caves visited: {:?}", caves_visited);
            // println!("can revisit: {}", can_revisit);
            path_so_far.push(current.clone());
            if current == "end" {
                // println!("---------------");
                // println!("FOUND: {:?}", path_so_far);
                Some(vec![path_so_far])
            } else {
                let empty_set = HashSet::new();
                let remove_set = caves_visited.clone();
                let paths_to_check: HashSet<&String> = layout.get(&current).unwrap_or(&empty_set).difference(&remove_set).collect();
                if paths_to_check.is_empty() {
                    None
                } else {
                    if current.chars().all(|c| c.is_lowercase()) {
                        if !can_revisit || current == "start" {
                            caves_visited.insert(current.clone());
                            Some(paths_to_check.iter().flat_map(|&next| find_paths_from(layout, next.clone(), path_so_far.clone(), caves_visited.clone(), can_revisit)).flatten().collect())
                        } else {
                            let mut no_more_revisit = paths_to_check.iter().flat_map(|&next| find_paths_from(layout, next.clone(), path_so_far.clone(), caves_visited.clone(), false)).flatten().collect::<Vec<Vec<String>>>();
                            caves_visited.insert(current.clone());
                            let mut still_revisit = paths_to_check.iter().flat_map(|&next| find_paths_from(layout, next.clone(), path_so_far.clone(), caves_visited.clone(), can_revisit)).flatten().collect::<Vec<Vec<String>>>();
                            no_more_revisit.append(&mut still_revisit);
                            Some(no_more_revisit)
                        }
                    } else {
                        Some(paths_to_check.iter().flat_map(|&next| find_paths_from(layout, next.clone(), path_so_far.clone(), caves_visited.clone(), can_revisit)).flatten().collect())
                    }
                }
            }
        }
        find_paths_from(&self.layout,"start".to_string(), Vec::new(), HashSet::new(), allow_single_return).unwrap()
    }
}


pub fn main() {
    let content = file_loading::read_file_to_string(2021, 12).unwrap();
    let graph = Graph::from_string(content);
    
    let path_count = graph.find_all_paths(false).len();
    println!("{}", path_count);

    let mut all_paths_with_return = graph.find_all_paths(true);
    all_paths_with_return.sort_by(|p1, p2| p1.join("").cmp(&p2.join("")));
    all_paths_with_return.dedup();
    println!("{}", all_paths_with_return.len());
    // let path_count_with_return = graph.find_all_paths(true).len();
    // println!("{}", path_count_with_return);
}
