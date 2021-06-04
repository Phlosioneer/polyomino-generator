
use std::cmp::Ordering;

use crate::polyominos::Polyomino;
use crate::symmetry::Symmetry;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord)]
pub struct Solution(Vec<&'static Polyomino>);

impl Solution {
    fn new(inner: Vec<&'static Polyomino>) -> Solution {
        Solution(inner)
    }
}

impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Solution) -> Option<Ordering> {
        if self.0.len() > other.0.len() {
            return Some(Ordering::Greater);
        } else if self.0.len() < other.0.len() {
            return Some(Ordering::Less);
        }

        for i in 0..self.0.len() {
            match self.0[i].cmp(&other.0[i]) {
                Ordering::Greater => return Some(Ordering::Greater),
                Ordering::Less => return Some(Ordering::Less),
                Ordering::Equal => ()
            }
        }

        Some(Ordering::Equal)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    cells: Vec<Option<usize>>,
    pub polyominos: Vec<&'static Polyomino>,
    width: i8,
    height: i8
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let mut cells = Vec::with_capacity(width * height);
        cells.resize(width * height, None);
        Board {
            cells,
            width: width as i8,
            height: height as i8,
            polyominos: Vec::new()
        }
    }

    pub fn from_solution(width: usize, height: usize, solution: &Solution) -> Board {
        let mut ret = Board::new(width, height);
        for &poly in &solution.0 {
            let success = ret.add(poly);
            assert_eq!(success, true, "Cannot fit piece {:?} into board: \n{}\nFull solution: {:?}", poly, ret.to_string(), solution);
        }
        ret
    }

    pub fn add(&mut self, poly: &'static Polyomino) -> bool {
        match self.try_add(poly) {
            Some(coords) => {
                self.add_at_position(poly, coords);
                true
            },
            None => false
        }
    }

    // Clones only if add is successful
    pub fn add_clone(&self, poly: &'static Polyomino) -> Option<Board> {
        self.try_add(poly)
            .map(|coords| {
                let mut ret = self.clone();
                ret.add_at_position(poly, coords);
                ret
            })
    }

    fn add_at_position(&mut self, poly: &'static Polyomino, base: (i8, i8)) {
        let (base_x, base_y) = base;
        for (poly_x, poly_y) in poly.coords() {
            self.set(base_x + poly_x, base_y + poly_y, Some(self.polyominos.len()));
        }
        self.polyominos.push(poly);
    }

    fn try_add(&self, poly: &'static Polyomino) -> Option<(i8, i8)> {
        self.find_first_open_cell()
            .map(|(base_x, base_y)| {
                for (poly_x, poly_y) in poly.coords() {
                    if self.get(base_x + poly_x, base_y + poly_y) != Some(None) {
                        return None;
                    }
                }
                Some((base_x, base_y))
            })
            .flatten()
    }

    fn find_first_open_cell(&self) -> Option<(i8, i8)> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == Some(None) {
                    return Some((x, y));
                }
            }
        }
        None
    }

    /// Outer option is None if out of bounds, inner option is None if
    /// cell is empty.
    pub fn get(&self, x: i8, y: i8) -> Option<Option<usize>> {
        if self.is_in_bounds(x, y) {
            Some(self.cells[x as usize + y as usize * self.width as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: i8, y: i8, value: Option<usize>) {
        if self.is_in_bounds(x, y) {
            let index = x as usize + y as usize * self.width as usize;
            assert_eq!(self.cells[index], None, "value: {:?}", value);
            self.cells[index] = value;
        } else {
            panic!();
        }
    }

    #[inline]
    fn is_in_bounds(&self, x: i8, y: i8) -> bool {
        !(x < 0 || y < 0 || x >= self.width || y >= self.height)
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.find_first_open_cell().is_none()
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(index) = self.get(x, y).flatten() {
                    ret += &index.to_string();
                } else {
                    ret += "?";
                }
            }
            if y != self.height - 1 {
                ret += "\n";
            }
        }
        ret
    }

    fn symmetric_board_polyominos(&self, symmetry: Symmetry) -> Solution {
        // Helper function
        let get_transformed = |mut x, mut y| {
            // The diagonal flip needs to be BEFORE the horizontal and vertical flips.
            // I don't really understand why, but it doesn't work if the diagonal is
            // done after the horizontal and vertical flips.
            if symmetry.diagonal {
                std::mem::swap(&mut x, &mut y);
            }
            if symmetry.horizontal {
                x = self.width - 1 - x;
            }
            if symmetry.vertical {
                y = self.height - 1 - y;
            }
            self.get(x, y).unwrap().unwrap()
        };

        if self.width != self.height {
            assert_eq!(symmetry.diagonal, false);
        }

        let mut indices = Vec::with_capacity(self.polyominos.len());
        for y in 0..self.height {
            for x in 0..self.width {
                let index = get_transformed(x, y);
                if !indices.contains(&index) {
                    indices.push(index);
                }
            }
        }

        let ret = Solution::new(indices.into_iter()
            .map(|index| self.polyominos[index].transform(symmetry))
            .collect());

        #[cfg(debug_assertions)]
        {
            // Confirm that the solution is formed correctly.
            Board::from_solution(self.width as usize, self.height as usize, &ret);
        }
        
        ret
    }

    pub fn cannonical_form(&self) -> Solution {
        assert_eq!(self.is_full(), true);

        let mut best_solution = None;
        for symmetry in Symmetry::ALL_SYMMETRIES {
            if self.width != self.height && symmetry.diagonal {
                continue;
            }
            let current_solution = self.symmetric_board_polyominos(symmetry);
            if let Some(ref mut best_solution) = best_solution {
                if &current_solution < best_solution {
                    *best_solution = current_solution;
                }
            } else {
                best_solution = Some(current_solution);
            }
        }
        best_solution.unwrap()
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod test {
    use super::*;
    use crate::polyominos::ALL_POLYOMINOS;

    fn find_poly(coords: Vec<(i8, i8)>) -> &'static Polyomino {
        for poly in ALL_POLYOMINOS.iter() {
            if poly.coords().map(|&c| c).collect::<Vec<_>>() == coords {
                return poly;
            }
        }
        panic!("Can't find poly with coords: {:?}", coords);
    }

    #[test]
    fn test_add() {
        // XX
        // X
        // X
        let leg = find_poly(vec![(0, 0), (0, 1), (0, 2), (1, 0)]);
        
        //  X
        // XX
        let corner = find_poly(vec![(-1, 1), (0, 0), (0, 1)]);

        // XX
        let flat = find_poly(vec![(0, 0), (1, 0)]);

        // 001
        // 011
        // 022
        let mut board = Board::new(3, 3);
        assert_eq!(board.add(leg), true);
        assert_eq!(board.add(corner), true);
        assert_eq!(board.add(flat), true);

        let expected_cells = vec![
            Some(0), Some(0), Some(1),
            Some(0), Some(1), Some(1),
            Some(0), Some(2), Some(2)
        ];
        assert_eq!(board.cells, expected_cells);
        assert_eq!(board.polyominos, vec![leg, corner, flat]);
        assert_eq!(board.is_full(), true);

        assert_eq!(board.to_string(), "001\n011\n022")
    }

    #[test]
    fn test_solution() {
        // XX
        // X
        // X
        let leg = find_poly(vec![(0, 0), (0, 1), (0, 2), (1, 0)]);
        
        //  X
        // XX
        let corner = find_poly(vec![(-1, 1), (0, 0), (0, 1)]);

        // XX
        let flat = find_poly(vec![(0, 0), (1, 0)]);

        // 001
        // 011
        // 022
        let board = Board::from_solution(3, 3, &Solution(vec![leg, corner, flat]));

        let no_change_symmetry = Symmetry::from_flips(false, false, false);
        assert_eq!(board.symmetric_board_polyominos(no_change_symmetry), Solution(vec![leg, corner, flat]));

        // X
        // X
        let flat = find_poly(vec![(0, 0), (0, 1)]);
        
        // XX
        // X
        let corner = find_poly(vec![(0, 0), (0, 1), (1, 0)]);

        //   X
        // XXX
        let leg = find_poly(vec![(-2, 1), (-1, 1), (0, 0), (0, 1)]);

        // 011
        // 012
        // 222
        let better_board = Board::from_solution(3, 3, &Solution(vec![flat, corner, leg]));

        let better_board_symmetry = Symmetry::from_flips(true, true, true);
        assert_eq!(board.symmetric_board_polyominos(better_board_symmetry).0, better_board.polyominos);

        assert_eq!(board.cannonical_form().0, better_board.polyominos);
        assert_eq!(better_board.cannonical_form().0, better_board.polyominos);
    }

    // TODO: Test this board:
    // 011
    // 112
    // 222
    //
    // and this board:
    // 000
    // 011
    // 112
    #[test]
    fn regression_test_solution() {
        // XXX
        // X
        let leg = find_poly(vec![(0, 0), (0, 1), (1, 0), (2, 0)]);
        
        //  XX
        // XX
        let zig = find_poly(vec![(-1, 1), (0, 0), (0, 1), (1, 0)]);

        // X
        let unit = find_poly(vec![(0, 0)]);

        // 000
        // 011
        // 112
        let board = Board::from_solution(3, 3, &Solution(vec![leg, zig, unit]));

        let no_change_symmetry = Symmetry::from_flips(false, false, false);
        assert_eq!(board.symmetric_board_polyominos(no_change_symmetry), Solution(vec![leg, zig, unit]));

        //  X
        //  X
        // XX
        let leg = find_poly(vec![(-1, 2), (0, 0), (0, 1), (0, 2)]);

        //  X
        // XX
        // X
        let zig = find_poly(vec![(-1, 1), (-1, 2), (0, 0), (0, 1)]);

        // 012
        // 112
        // 122
        let better_board = Board::from_solution(3, 3, &Solution(vec![unit, zig, leg]));

        let better_board_symmetry = Symmetry::from_flips(true, true, true);
        assert_eq!(board.symmetric_board_polyominos(better_board_symmetry).0, better_board.polyominos);

        assert_eq!(board.cannonical_form().0, better_board.polyominos);
        assert_eq!(better_board.cannonical_form().0, better_board.polyominos);
    }
}