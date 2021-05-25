
use crate::polyominos::Polyomino;

#[derive(Debug, Clone)]
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
}