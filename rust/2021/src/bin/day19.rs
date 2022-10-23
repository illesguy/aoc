use commons::file_loading;
use std::collections::{HashSet, HashMap};


#[derive(Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Point {
    fn from_string(content: &str) -> Self {
        let coordinates = content.split(",").map(|n| n.parse::<i32>().unwrap()).take(3).collect::<Vec<i32>>();
        let (x, y, z) = (coordinates[0], coordinates[1], coordinates[2]);
        Point {x, y, z}
    }

    fn distance(&self, other: &Point) -> i32 {
        (self.x + other.x).pow(2) + (self.y + other.y).pow(2) + (self.z + other.z).pow(2)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl std::hash::Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}


struct Scanner {
    beacons: HashSet<Point>
}

impl Scanner {
    fn from_string(content: &str) -> Self {
        let beacons = content.split("\n").skip(1).map(Point::from_string).collect();
        Scanner {beacons}
    }

    fn add(self, other: Scanner) -> Scanner {
        let mut distances: HashMap<&Point, HashSet<i32>> = HashMap::new();
        for i in 0..self.beacons.len() - 1 {
            let p1 = self.beacons.get(i).unwrap();
            for j in i + 1..self.beacons.len() {
                let p2 = self.beacons.get(j).unwrap();
                distances.insert((p1, p2), p1.distance(p2));
            }
        }

        self
    }
}


pub fn main() {
    let content = file_loading::read_file_to_string(2021, 19).unwrap();
    let scanners = content.split("\n\n").map(Scanner::from_string).collect::<Vec<Scanner>>();
    let summed_scanner = scanners.into_iter().reduce(|a, b| a.add(b)).unwrap();

    let beacon_count = summed_scanner.beacons.len();
    println!("{}", beacon_count)
}
