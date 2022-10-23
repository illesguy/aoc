use commons::file_loading;
use std::collections::VecDeque;


struct CodeBoard {
    dots: Vec<Vec<bool>>
}


impl CodeBoard {
    fn from_string(dot_coords: &str) -> CodeBoard {
        let coords_list: Vec<(usize, usize)> = dot_coords.split("\n").map(|c| {
            let (x, y) = c.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        }).collect();

        let col_count = coords_list.iter().map(|&(x, _)| x).max().unwrap() + 1;
        let row_count = coords_list.iter().map(|&(_, y)| y).max().unwrap() + 1;

        let mut dots = vec![vec![false; row_count]; col_count];
        for &(x, y) in coords_list.iter() {
            dots[x][y] = true;
        }

        CodeBoard {
            dots
        }
    }

    fn get_dot_count(&self) -> usize {
        self.dots.iter().map(|col| col.iter().filter(|&&v| v).count()).sum()
    }

    fn fold_up(&mut self, row: usize) {
        for col in self.dots.iter_mut() {
            for i in 0..row {
                col[i] |= col[col.len() - 1 - i]
            }
            col.truncate(row)
        }
    }

    fn fold_left(&mut self, col: usize) {
        for i in 0..col {
            let len = self.dots.len();
            let fold = self.dots[len - 1 - i].clone();
            let curr = &mut self.dots[i];
            for j in 0..curr.len() {
                curr[j] |= fold[j]
            }
        }
        self.dots.truncate(col)
    }

    fn fold(&mut self, fold: &Fold) {
        match fold {
            Fold::Left(col) => self.fold_left(*col),
            Fold::Up(row) => self.fold_up(*row)
        }
    }

    fn print_code(&self) {
        for i in 0..self.dots[0].len() {
            for j in 0..self.dots.len() {
                if self.dots[j][i] {
                    print!("##");
                } else {
                    print!("  ");
                }
            }
            println!("");
        }
    }
}


#[derive(Debug)]
enum Fold {
    Up(usize),
    Left(usize)
}


fn parse_folds(folds: &str) -> VecDeque<Fold> {
    folds.split("\n").map(|ins| {
        let (f, c) = ins[11..ins.len()].split_once("=").unwrap();
        if f == "x" {
            Fold::Left(c.parse().unwrap())
        } else {
            Fold::Up(c.parse().unwrap())
        }
    }).collect()
}


pub fn main() {
    let content = file_loading::read_file_to_string(2021, 13).unwrap();
    let (dot_coords, folds) = content.split_once("\n\n").unwrap();

    let mut code_board = CodeBoard::from_string(dot_coords);
    let mut instructions = parse_folds(folds);

    let first_fold = instructions.pop_front().unwrap();
    code_board.fold(&first_fold);
    println!("{}", code_board.get_dot_count());
    

    for fold in instructions.iter() {
        code_board.fold(fold)
    }

    code_board.print_code();
}