
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Symmetry {
    pub horizontal: bool,
    pub vertical: bool,
    pub diagonal: bool
}

impl Symmetry {
    const HORIZONTAL_MASK: usize = 0b001;
    const VERTICAL_MASK: usize = 0b010;
    const DIAGONAL_MASK: usize = 0b100;

    pub const ALL_SYMMETRIES: [Symmetry; 8] = [
        Symmetry::from_index_unchecked(0),
        Symmetry::from_index_unchecked(1),
        Symmetry::from_index_unchecked(2),
        Symmetry::from_index_unchecked(3),
        Symmetry::from_index_unchecked(4),
        Symmetry::from_index_unchecked(5),
        Symmetry::from_index_unchecked(6),
        Symmetry::from_index_unchecked(7)
    ];

    pub fn from_flips(horizontal: bool, vertical: bool, diagonal: bool) -> Symmetry {
        Symmetry { horizontal, vertical, diagonal }
    }

    const fn from_index_unchecked(index: usize) -> Symmetry {
        Symmetry {
            horizontal: index & Self::HORIZONTAL_MASK != 0,
            vertical: index & Self::VERTICAL_MASK != 0,
            diagonal: index & Self::DIAGONAL_MASK != 0
        }
    }

    pub fn from_index(index: usize) -> Symmetry {
        if index > 8 {
            panic!();
        }
        Self::from_index_unchecked(index)
    }

    pub fn into_index(self) -> usize {
        let mut ret = 0;
        if self.horizontal {
            ret |= Self::HORIZONTAL_MASK;
        }
        if self.vertical {
            ret |= Self::VERTICAL_MASK;
        }
        if self.diagonal {
            ret |= Self::DIAGONAL_MASK;
        }
        ret
    }

    pub fn mirror_horizontal(mut self) -> Symmetry {
        if self.diagonal {
            // Diagonal flip happens AFTER horizontal and vertical flips.
            // So if we want to flip the output horizontally, we need to flip
            // the input vertically.
            self.vertical = !self.vertical;
        } else {
            // Just a simple toggle.
            self.horizontal = !self.horizontal;
        }
        self
    }
    
    pub fn mirror_vertical(mut self) -> Symmetry {
        if self.diagonal {
            // Diagonal flip happens AFTER horizontal and vertical flips.
            // So if we want to flip the output vertically, we need to flip
            // the input horizontally.
            self.horizontal = !self.horizontal;
        } else {
            self.vertical = !self.vertical;
        }
        self
    }

    pub fn rotate(mut self, mut clockwise: i8) -> Symmetry {
        clockwise %= 4;
        if clockwise < 0 {
            clockwise += 4;
        }
        
        for _ in 0..clockwise {
            // Rotate 90 degrees clockwise.
            // Always toggle the diagonal; the rule for flipping horizontal or
            // vertical was figured out by trial and error.
            if self.diagonal {
                self.diagonal = false;
                self.horizontal = !self.horizontal;
            } else {
                self.diagonal = true;
                self.vertical = !self.vertical;
            }
        }
        self
    }
}