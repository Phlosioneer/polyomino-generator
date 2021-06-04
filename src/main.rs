
mod polyominos;
mod board;
mod symmetry;
use polyominos::Polyomino;
use board::Board;

use std::collections::BTreeSet;

const MAX_ONES_OR_TWOS: u8 = 1;
const MAX_THREES: u8 = 2;
const WIDTH: usize = 6;
const HEIGHT: usize = 6;

#[derive(Debug, Clone)]
struct RestrictedBoard {
    inner: Board,
    one_or_two_sized_count: u8,
    three_sized_count: u8,
}

impl RestrictedBoard {
    pub fn new(width: usize, height: usize) -> RestrictedBoard {
        RestrictedBoard {
            inner: Board::new(width, height),
            one_or_two_sized_count: 0,
            three_sized_count: 0
        }
    }

    pub fn add_clone(&self, poly: &'static Polyomino) -> Option<RestrictedBoard> {
        let is_tiny = poly.size() == 1 || poly.size() == 2;
        let is_three = poly.size() == 3;
        if is_tiny && self.one_or_two_sized_count >= MAX_ONES_OR_TWOS {
            return None;
        } else if is_three && self.three_sized_count >= MAX_THREES {
            return None;
        }
        self.inner.add_clone(poly).map(|inner| {
            let mut one_or_two_sized_count = self.one_or_two_sized_count;
            let mut three_sized_count = self.three_sized_count;
            if is_tiny {
                one_or_two_sized_count += 1;
            } else if is_three {
                three_sized_count += 1;
            }
            RestrictedBoard { inner, one_or_two_sized_count, three_sized_count }
        })
    }

    pub fn board(&self) -> &Board {
        &self.inner
    }
}

fn main() {
    let mut stack = vec![RestrictedBoard::new(WIDTH, HEIGHT)];
    let mut completed_boards = BTreeSet::new();

    while let Some(board) = stack.pop() {
        for polyomino in polyominos::ALL_POLYOMINOS.iter() {
            if let Some(new_board) = board.add_clone(polyomino) {
                if new_board.board().is_full() {
                    let changed = completed_boards.insert(new_board.board().cannonical_form());
                    if changed {
                        let should_print;
                        if completed_boards.len() < 10 {
                            should_print = true;
                        } else if completed_boards.len() < 100 {
                            should_print = completed_boards.len() % 10 == 0;
                        } else if completed_boards.len() < 1000 {
                            should_print = completed_boards.len() % 100 == 0;
                        } else if completed_boards.len() < 10000 {
                            should_print = completed_boards.len() % 1000 == 0;
                        } else if completed_boards.len() < 100000 {
                            should_print = completed_boards.len() % 10000 == 0;
                        } else {
                            should_print = completed_boards.len() % 100000 == 0;
                        }
                        if should_print {
                            println!("{}", completed_boards.len());
                        }
                    }
                } else {
                    stack.push(new_board);
                }
            }
        }
    }

    //for solution in &completed_boards {
       //println!("----\n{}\n----\n\n", Board::from_solution(WIDTH, HEIGHT, &solution).to_string());
    //}
    println!("{}", completed_boards.len());
}