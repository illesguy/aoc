use commons::file_loading;
use std::collections::HashMap;


#[derive(Debug)]
struct GasVent {
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
    pub is_diagonal: bool,
}


impl GasVent {
    fn from_string(s: &str) -> GasVent {
        let mut parts = s.split(" -> ").into_iter();
        let starts = parts.next().unwrap();
        let ends = parts.next().unwrap();

        let mut start_parts = starts.split(",").into_iter();
        let x_start = start_parts.next().unwrap().parse::<i32>().unwrap();
        let y_start = start_parts.next().unwrap().parse::<i32>().unwrap();

        let mut end_parts = ends.split(",").into_iter();
        let x_end = end_parts.next().unwrap().parse::<i32>().unwrap();
        let y_end = end_parts.next().unwrap().parse::<i32>().unwrap();

        let is_diagonal = x_start != x_end && y_start != y_end;
        GasVent {
            x_start,
            x_end,
            y_start,
            y_end,
            is_diagonal,
        }
    }

    fn area_covered(&self) -> Vec<(i32, i32)> {
        let x_decreasing = self.x_start > self.x_end;
        let x_range = if x_decreasing {
            (self.x_end..=self.x_start).rev().collect::<Vec<i32>>()
        } else {
            (self.x_start..=self.x_end).collect::<Vec<i32>>()
        };

        let y_decreasing = self.y_start > self.y_end;
        let y_range = if y_decreasing {
            (self.y_end..=self.y_start).rev().collect::<Vec<i32>>()
        } else {
            (self.y_start..=self.y_end).collect::<Vec<i32>>()
        };

        x_range.iter().flat_map(|&x| {
            y_range.iter().map(move |&y| (x, y))
        }).filter(move |&(x, y)| {
            !self.is_diagonal ||
            (x + y == self.x_start + self.y_start && x + y == self.x_end + self.y_end) ||
            (x - y == self.x_start - self.y_start && x - y == self.x_end - self.y_end)
        }).collect::<Vec<(i32, i32)>>()
    }
}


fn main() {
    let content = file_loading::read_file_to_string(2021, 5).unwrap();
    let vents = content.split("\n").map(|line| GasVent::from_string(line)).collect::<Vec<GasVent>>();

    let mut vent_map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut diag_vent_map: HashMap<(i32, i32), i32> = HashMap::new();
    for vent in vents {
        for cover in vent.area_covered() {
            let count = diag_vent_map.entry(cover).or_insert(0);
            *count += 1;
        }

        if !vent.is_diagonal {
            for cover in vent.area_covered() {
                let count = vent_map.entry(cover).or_insert(0);
                *count += 1;
            }   
        }
    }

    let overlaps = vent_map.iter().filter(|&(_, &v)| v >= 2).count();
    println!("{}", overlaps);
    let diag_overlaps = diag_vent_map.iter().filter(|&(_, &v)| v >= 2).count();
    println!("{}", diag_overlaps);
}
