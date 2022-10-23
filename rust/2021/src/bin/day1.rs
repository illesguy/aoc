use commons::file_loading;


fn main() {
    let mut prev: Option<i32> = None;
    let mut increases = 0;
    let mut prev_prev: Option<i32> = None;
    let mut prev_window: Option<i32> = None;
    let mut window_increases = 0;

    if let Ok(lines) = file_loading::load_file(2021, 1) {
        for line in lines {
            if let Ok(d) = line {
                let i = d.parse::<i32>().unwrap();
                if let Some(p) = prev {
                    if p < i {
                        increases += 1;
                    }
                    if let Some(pp) = prev_prev {
                        if let Some(pw) = prev_window {
                            if pw < i + p + pp {
                                window_increases += 1;
                            }
                        }
                        prev_window = Some(i + p + pp);
                    }
                }
                prev_prev = prev;
                prev = Some(i);
            }
        }
    }

    println!("{}", increases);
    println!("{}", window_increases);
}
