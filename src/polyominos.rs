
use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter::FromIterator;
use tinyvec::ArrayVec;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ALL_POLYOMINOS: Vec<Polyomino> = generate_all_polyominos(4);
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Polyomino {
    coords: ArrayVec<[(i8, i8); 4]>,
}

impl Polyomino {
    fn new(coords: &[(i8, i8)]) -> Polyomino {
        if coords.len() > 4 {
            panic!();
        }
        if !coords.contains(&(0, 0)) {
            panic!();
        }
        let mut actual_coords = ArrayVec::from_iter(coords.iter().map(|coord| *coord));
        actual_coords.sort_by(|(ax, ay): &(i8, i8), (bx, by)| {
            let ret = ax.cmp(bx);
            if ret == Ordering::Equal {
                ay.cmp(by)
            } else {
                ret
            }
        });

        Polyomino {
            coords: actual_coords
        }
    }

    pub fn size(&self) -> u8 {
        self.coords.len() as u8
    }

    pub fn signed_size(&self) -> i8 {
        self.coords.len() as i8
    }

    pub fn coords(&self) -> impl Iterator<Item = &(i8, i8)> {
        self.coords.iter()
    }
}

// Sorted smallest-first
fn generate_all_polyominos(max_size: usize) -> Vec<Polyomino> {
    let mut stack = Vec::new();
    let mut polyominos = HashSet::new();

    let base = vec![(0, 0)];
    polyominos.insert(Polyomino::new(&base));
    if max_size > 1 {
        stack.push(base);
    }

    while let Some(polyomino) = stack.pop() {
        for coord in adjacent_coords(&polyomino) {
            let mut new_poly = polyomino.clone();
            new_poly.push(coord);
            polyominos.insert(Polyomino::new(&new_poly));
            if new_poly.len() < max_size {
                stack.push(new_poly);
            }
        }
    }

    let mut ret: Vec<_> = polyominos.into_iter().collect();
    ret.sort_by_key(|p| p.size());
    ret
}

fn adjacent_coords(polyomino: &[(i8, i8)]) -> Vec<(i8, i8)> {
    let mut ret = HashSet::new();
    for &(x, y) in polyomino {
        ret.insert((x - 1, y));
        ret.insert((x + 1, y));
        ret.insert((x, y - 1));
        ret.insert((x, y + 1));
    }
    ret.into_iter()
        .filter(|coord| !polyomino.contains(coord))
        .filter(|&(_, y)| y >= 0)
        .filter(|&(x, y)| !(y == 0 && x < 0))
        .collect()
}