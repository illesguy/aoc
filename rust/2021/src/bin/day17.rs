use commons::file_loading;


struct Area {
    x_lower: i32,
    x_upper: i32,
    y_lower: i32,
    y_upper: i32
}


impl Area {
    fn from_string(desc: String) -> Area {
        let (x_area, y_area) = desc.split_once(", ").unwrap();
        let (x_lower, x_upper) = x_area[15..].split_once("..").unwrap();
        let (y_lower, y_upper) = y_area[2..].split_once("..").unwrap();

        Area {
            x_lower: x_lower.parse::<i32>().unwrap(),
            x_upper: x_upper.parse::<i32>().unwrap(),
            y_lower: y_lower.parse::<i32>().unwrap(),
            y_upper: y_upper.parse::<i32>().unwrap()
        }
    }

    fn contains_point(&self, p: &Point) -> bool {
        p.x >= self.x_lower && p.x <= self.x_upper && p.y >= self.y_lower && p.y <= self.y_upper
    }

    fn cant_reach(&self, p: &Point, t: &Trajectory) -> bool {
        (p.x > self.x_upper && t.x >= 0) || (p.y < self.y_lower && t.y < 0)
    }
}


struct Point {
    x: i32,
    y: i32
}


impl Point {
    fn move_by(&mut self, trajectory: &Trajectory) {
        self.x += trajectory.x;
        self.y += trajectory.y;
    }
}


struct Trajectory {
    x: i32,
    y: i32
}


impl Trajectory {
    fn weaken_trajectory(&mut self) {
        if self.x < 0 {
            self.x += 1;
        } else if self.x > 0 {
            self.x -= 1;
        }

        self.y -= 1;
    }
}


fn trajectory_reaches_area(mut t: Trajectory, target_area: &Area) -> bool {
    let mut p = Point {
        x: 0,
        y: 0
    };

    loop {
        if target_area.contains_point(&p) {
            return true
        }
        if target_area.cant_reach(&p, &t) {
            return false
        }
        p.move_by(&t);
        t.weaken_trajectory();
    }
}


pub fn main() {
    let content = file_loading::read_file_to_string(2021, 17).unwrap();
    let target_area = Area::from_string(content);

    let max_height = target_area.y_lower * (target_area.y_lower - 1) / 2;
    println!("{}", max_height);

    let mut t_count = 0;

    for x in 0..=target_area.x_upper {
        for y in target_area.y_lower..=-target_area.y_lower {
            if trajectory_reaches_area(Trajectory {x, y}, &target_area) {
                t_count += 1;
            }
        }
    }

    println!("{}", t_count);
}
