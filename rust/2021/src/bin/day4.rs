use commons::file_loading;


#[derive(Debug)]
struct BingoBoard {
    elements: Vec<(i32, bool)>,
    size: usize,
}

impl BingoBoard {
    fn from_string(s: &str) -> BingoBoard {
        let rows = s.split("\n");
        let mut size = 0;
        let elements = rows.flat_map(|row| {
            size += 1;
            row.split_whitespace().map(|n| {
                (n.parse::<i32>().unwrap(), false)
            })
        }).collect();
        
        BingoBoard {
            elements: elements,
            size: size
        }
    }

    fn mark_number(&mut self, number: i32) -> () {
        let elements = &mut self.elements;

        for i in 0..elements.len() {
            elements[i].1 = elements[i].1 || elements[i].0 == number;
        }
    }

    fn sum_non_marked(&self) -> i32 {
        self.elements.iter().filter(|(_, marked)| !marked).map(|(value, _)| value).sum()
    }

    fn is_complete(&self) -> bool {
        for i in 0..self.size {
            let row_complete = self.elements.iter().enumerate().filter(|&(index, _)| index / self.size == i).all(|(_, (_, marked))| *marked);
            let col_complete = self.elements.iter().enumerate().filter(|&(index, _)| index % self.size == i).all(|(_, (_, marked))| *marked);

            if row_complete || col_complete {
                return true;
            }
        }

        false
    }
}


fn find_winning_score(numbers: &Vec<i32>, boards: &mut Vec<&mut BingoBoard>) -> i32 {
    for &number in numbers {
        for board in boards.iter_mut() {
            board.mark_number(number);
            if board.is_complete() {
                return number * board.sum_non_marked();
            }
        }
    }
    panic!("No board gets completed from the numbers")
}


fn find_last_winning_score(numbers: &Vec<i32>, boards: &mut Vec<&mut BingoBoard>) -> i32 {
    let mut winning_score = 0;
    for &number in numbers {
        for board in boards.iter_mut() {
            if board.is_complete() {
                continue;
            }

            board.mark_number(number);
            if board.is_complete() {
                winning_score = number * board.sum_non_marked();
            }
        }
    }
    winning_score
}


fn main() {
    let content = file_loading::read_file_to_string(2021, 4).unwrap();
    let parts = content.split("\n\n").collect::<Vec<&str>>();

    let numbers = parts[0].clone().split(",").map(|n| n.parse::<i32>().unwrap()).collect();
    let mut boards = parts[1..].iter().map(|&b| BingoBoard::from_string(b)).collect::<Vec<BingoBoard>>();
    let mut board_references = boards.iter_mut().collect();

    let score = find_winning_score(&numbers, &mut board_references);
    println!("{}", score);

    let last_score = find_last_winning_score(&numbers, &mut board_references);
    println!("{}", last_score);
}
