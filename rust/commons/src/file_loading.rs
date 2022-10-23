use std::fs::File;
use std::io::{self, BufRead, Error, Read};


pub fn load_file(year: i16, day: i8) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(format!("../inputs/{}/day{}.txt", year, day))?;
    Ok(io::BufReader::new(file).lines())
}


pub fn read_file_to_string(year: i16, day: i8) -> Result<String, Error> {
    match File::open(format!("../inputs/{}/day{}.txt", year, day)) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            Ok(content)
        },
        Err(err) => {
            Err(err)
        }
    }
}
