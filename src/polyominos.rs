
use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::hash::{Hash, Hasher};
use tinyvec::ArrayVec;
use crate::symmetry::Symmetry;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ALL_POLYOMINOS: Vec<Polyomino> = generate_all_polyominos(4);
}

#[derive(Debug, Clone, Eq, Ord)]
pub struct Polyomino {
    coords: ArrayVec<[(i8, i8); 4]>,
    symmetries: Option<[usize; 8]>
}

impl Polyomino {
    fn coord_sort(first: &(i8, i8), second: &(i8, i8)) -> Ordering {
        let (ax, ay) = first;
        let (bx, by) = second;
        let ret = ax.cmp(bx);
        if ret == Ordering::Equal {
            ay.cmp(by)
        } else {
            ret
        }
    }

    fn new(coords: &[(i8, i8)]) -> Polyomino {
        if coords.len() > 4 {
            panic!();
        }
        if !coords.contains(&(0, 0)) {
            panic!();
        }
        let mut actual_coords = ArrayVec::from_iter(coords.iter().map(|coord| *coord));
        actual_coords.sort_by(Self::coord_sort);

        Polyomino {
            coords: actual_coords,
            symmetries: None
        }
    }

    #[inline]
    pub fn size(&self) -> u8 {
        self.coords.len() as u8
    }

    #[inline]
    pub fn coords(&self) -> impl Iterator<Item = &(i8, i8)> {
        self.coords.iter()
    }

    fn apply_flips(&mut self, transform: Symmetry) {
        // First apply the transformations.
        for (x, y) in &mut self.coords {
            if transform.horizontal {
                *x *= -1;
            }
            if transform.vertical {
                *y *= -1;
            }
            if transform.diagonal {
                std::mem::swap(x, y);
            }
        }

        // Then move the coordinates so that the top-left most coordinate moves to (0, 0).
        // "top-left most" means the with the smallest x on the row with the smallest y (so
        // y is more important).
        let mut top_left_most_coord = (i8::MAX, i8::MAX);
        for &(x, y) in &self.coords {
            if y < top_left_most_coord.1 {
                top_left_most_coord = (x, y);
            } else if y == top_left_most_coord.1 && x < top_left_most_coord.0 {
                top_left_most_coord = (x, y);
            }
        }

        for (x, y) in &mut self.coords {
            *x -= top_left_most_coord.0;
            *y -= top_left_most_coord.1;
        }

        // Finally sort the coords
        self.coords.sort_by(Self::coord_sort);
    }

    pub fn to_string(&self) -> String {
        let mut max_x = i8::MIN;
        let mut max_y = i8::MIN;
        let mut min_x = i8::MAX;

        for &(x, y) in &self.coords {
            max_x = i8::max(max_x, x);
            max_y = i8::max(max_y, y);
            min_x = i8::min(min_x, x);
        }

        let mut ret = String::new();
        for y in 0..=max_y {
            for x in min_x ..= max_x {
                if self.coords.contains(&(x, y)) {
                    if (x, y) == (0, 0) {
                        ret += "@";
                    } else {
                        ret += "#";
                    }
                } else {
                    ret += " ";
                }
            }
            ret += "\n";
        }
        ret
    }

    // Can't be mutable because it needs to access the array that contains itself.
    fn compute_transforms(&self, all_polyominos: &[Polyomino]) -> [usize; 8] {
        let mut matching_polyominos = Vec::with_capacity(8);
        matching_polyominos.resize(8, self.clone());

        for (i, poly) in matching_polyominos.iter_mut().enumerate() {
            poly.apply_flips(Symmetry::ALL_SYMMETRIES[i]);
        }

        let mut indices: [usize; 8] = Default::default();
        for (i, poly) in matching_polyominos.into_iter().enumerate() {
            let index = match all_polyominos.iter().position(|e| e == &poly) {
                Some(p) => p,
                None => {
                    panic!("Could not find poly: \n{}({:?})\n\nSimilar polyominos:\n{:?}", poly.to_string(), poly.coords,
                        all_polyominos.iter().filter(|p| p.size() == poly.size()).map(|p| p.to_string()).collect::<Vec<_>>())
                }
            };
            indices[i] = index;
        }

        indices
    }

    pub fn transform(&self, symmetry: Symmetry) -> &'static Polyomino {
        &ALL_POLYOMINOS[self.symmetries.unwrap()[symmetry.into_index()]]
    }
}

impl PartialEq for Polyomino {
    fn eq(&self, other: &Polyomino) -> bool {
        // Ignore the symmetry field, which is more like a cache than a part of the poly.
        self.coords == other.coords
    }
}

// Matching hash function for custom PartialEq implementation
impl Hash for Polyomino {
    fn hash<H>(&self, state: &mut H)
    where H: Hasher {
        self.coords.hash(state);
    }
}

impl PartialOrd for Polyomino {
    fn partial_cmp(&self, other: &Polyomino) -> Option<Ordering> {
        if self.coords.len() > other.coords.len() {
            return Some(Ordering::Greater);
        } else if self.coords.len() < other.coords.len() {
            return Some(Ordering::Less);
        }

        for i in 0..self.coords.len() {
            match Polyomino::coord_sort(&self.coords[i], &other.coords[i]) {
                Ordering::Greater => return Some(Ordering::Greater),
                Ordering::Less => return Some(Ordering::Less),
                Ordering::Equal => ()
            }
        }

        Some(Ordering::Equal)
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
    ret.sort();

    for i in 0..ret.len() {
        let symmetries = ret[i].compute_transforms(&ret);
        assert_eq!(symmetries[0], i);
        ret[i].symmetries = Some(symmetries);
    }

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

#[allow(unused_imports)]
#[cfg(test)]
mod test {
    use super::*;

    fn find_global_poly(coords: Vec<(i8, i8)>) -> &'static Polyomino {
        for poly in ALL_POLYOMINOS.iter() {
            if poly.coords().map(|&c| c).collect::<Vec<_>>() == coords {
                return poly;
            }
        }
        panic!();
    }

    #[test]
    fn test_coord_sort() {
        let test = vec![(0, 1), (1, 0), (0, 0)];
        let sorted = vec![(0, 0), (0, 1), (1, 0)];
        
        let poly = Polyomino::new(&test);
        assert_eq!(poly.coords.to_vec(), sorted);

        let test = vec![(1, 0), (0, 0), (0, 2), (0, 1)];
        let sorted = vec![(0, 0), (0, 1), (0, 2), (1, 0)];

        let poly = Polyomino::new(&test);
        assert_eq!(poly.coords.to_vec(), sorted);

        let test = vec![(0, 0)];
        let sorted = vec![(0, 0)];

        let poly = Polyomino::new(&test);
        assert_eq!(poly.coords.to_vec(), sorted);
    }

    #[test]
    fn test_flips() {
        // X
        // XXX
        let leg = vec![(0, 0), (0, 1), (1, 1), (2, 1)];

        //   X
        // XXX
        let h_leg = vec![(-2, 1), (-1, 1), (0, 0), (0, 1),];
        
        // XXX
        // X
        let v_leg = vec![(0, 0), (0, 1), (1, 0), (2, 0)];

        // XXX
        //   X
        let hv_leg = vec![(0, 0), (1, 0), (2, 0), (2, 1)];

        // XX
        //  X
        //  X
        let d_leg = vec![(0, 0), (1, 0), (1, 1), (1, 2)];

        //  X
        //  X
        // XX
        let hd_leg = vec![(-1, 2), (0, 0), (0, 1), (0, 2)];

        // XX
        // X
        // X
        let vd_leg = vec![(0, 0), (0, 1), (0, 2), (1, 0)];

        // X
        // X
        // XX
        let hvd_leg = vec![(0, 0), (0, 1), (0, 2), (1, 2)];

        let poly = Polyomino::new(&leg);
        let mut h_poly = poly.clone();
        let mut v_poly = poly.clone();
        let mut hv_poly = poly.clone();
        let mut d_poly = poly.clone();
        let mut hd_poly = poly.clone();
        let mut vd_poly = poly.clone();
        let mut hvd_poly = poly.clone();
        // Note: apply_flips(horizontal, vertical, diagonal)
        h_poly.apply_flips(Symmetry::from_flips(true, false, false));
        v_poly.apply_flips(Symmetry::from_flips(false, true, false));
        hv_poly.apply_flips(Symmetry::from_flips(true, true, false));
        d_poly.apply_flips(Symmetry::from_flips(false, false, true));
        hd_poly.apply_flips(Symmetry::from_flips(true, false, true));
        vd_poly.apply_flips(Symmetry::from_flips(false, true, true));
        hvd_poly.apply_flips(Symmetry::from_flips(true, true, true));

        assert_eq!(h_poly.coords.to_vec(), h_leg);
        assert_eq!(v_poly.coords.to_vec(), v_leg);
        assert_eq!(hv_poly.coords.to_vec(), hv_leg);
        assert_eq!(d_poly.coords.to_vec(), d_leg);
        assert_eq!(hd_poly.coords.to_vec(), hd_leg);
        assert_eq!(vd_poly.coords.to_vec(), vd_leg);
        assert_eq!(hvd_poly.coords.to_vec(), hvd_leg);

        let all_poly = vec![
            poly, h_poly, v_poly, hv_poly,
            d_poly, hd_poly, vd_poly, hvd_poly
        ];
        let transforms = all_poly[0].compute_transforms(&all_poly);
        let index = Symmetry::from_flips(false, false, false).into_index();
        let h_index = Symmetry::from_flips(true, false, false).into_index();
        let v_index = Symmetry::from_flips(false, true, false).into_index();
        let hv_index = Symmetry::from_flips(true, true, false).into_index();
        let d_index = Symmetry::from_flips(false, false, true).into_index();
        let hd_index = Symmetry::from_flips(true, false, true).into_index();
        let vd_index = Symmetry::from_flips(false, true, true).into_index();
        let hvd_index = Symmetry::from_flips(true, true, true).into_index();
        assert_eq!(transforms[index], 0);
        assert_eq!(transforms[h_index], 1);
        assert_eq!(transforms[v_index], 2);
        assert_eq!(transforms[hv_index], 3);
        assert_eq!(transforms[d_index], 4);
        assert_eq!(transforms[hd_index], 5);
        assert_eq!(transforms[vd_index], 6);
        assert_eq!(transforms[hvd_index], 7);
    }

    #[test]
    fn test_to_string() {
        // X
        // XX
        //  X
        let test = vec![(0, 0), (0, 1), (1, 1), (1, 2)];
        let output = "@ \n##\n #\n";

        let poly = Polyomino::new(&test);
        assert_eq!(poly.to_string(), output);
    }

    #[test]
    fn test_transform() {
        // X
        // X
        // XX
        let original = find_global_poly(vec![(0, 0), (0, 1), (0, 2), (1, 2)]);

        //  X
        //  X
        // XX
        let h_flip = find_global_poly(vec![(-1, 2), (0, 0), (0, 1), (0, 2)]);

        assert_eq!(original.transform(Symmetry::from_flips(true, false, false)), h_flip);
        assert_eq!(h_flip.transform(Symmetry::from_flips(true, false, false)), original);

        // XX
        // X
        // X
        let v_flip = find_global_poly(vec![(0, 0), (0, 1), (0, 2), (1, 0)]);
        
        assert_eq!(original.transform(Symmetry::from_flips(false, true, false)), v_flip);
        assert_eq!(v_flip.transform(Symmetry::from_flips(false, true, false)), original);

        // XXX
        //   X
        let d_flip = find_global_poly(vec![(0, 0), (1, 0), (2, 0), (2, 1)]);
        
        assert_eq!(original.transform(Symmetry::from_flips(false, false, true)), d_flip);
        assert_eq!(d_flip.transform(Symmetry::from_flips(false, false, true)), original);
    }

    #[test]
    fn test_comparison() {
        let small_tall = Polyomino::new(&vec![(0, 0), (0, 1)]);
        let big_square = Polyomino::new(&vec![(0, 0), (0, 1), (1, 0), (1, 1)]);
        assert_eq!(small_tall < big_square, true, "{:?} < {:?}", small_tall, big_square);

        let small_flat = Polyomino::new(&vec![(0, 0), (1, 0)]);
        assert_eq!(small_tall < small_flat, true, "{:?} < {:?}", small_tall, small_flat);

        let mut all = vec![small_flat.clone(), big_square.clone(), small_tall.clone()];
        all.sort();
        assert_eq!(all, vec![small_tall, small_flat, big_square]);

        //  XX
        // XX
        let zig_wide = Polyomino::new(&vec![(-1, 1), (0, 0), (0, 1), (1, 0)]);

        //  X
        // XX
        // X
        let zig_tall = Polyomino::new(&vec![(-1, 1), (-1, 2), (0, 0), (0, 1)]);
        assert_eq!(zig_tall < zig_wide, true, "{:?} < {:?}", zig_tall, zig_wide);
    }
}