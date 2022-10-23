use commons::file_loading;


fn main() {
    let mut hp = 0;
    let mut d = 0;

    let mut hp2 = 0;
    let mut d2 = 0;
    let mut a = 0;

    if let Ok(lines) = file_loading::load_file(2021, 2) {
        for line in lines {
            if let Ok(lin) = line {
                let mut parts = lin.split_whitespace();
                let dir = parts.next().unwrap();
                let amount = parts.next().unwrap().parse::<i32>().unwrap();
                match dir {
                    "forward" => {
                        hp += amount;
                        hp2 += amount;
                        d2 += a * amount;
                    },
                    "down" => {
                        d += amount;
                        a += amount;
                    },
                    "up" => {
                        d -= amount;
                        a -= amount;
                    },
                    _ => panic!("unknown direction {}", dir),
                }
            }
        }
    }

    println!("{}", hp * d);
    println!("{}", hp2 * d2);
}
