/// The base type for what fills in a sudoku square. An enum instead of a raw integer/wrapper struct for ~*~ type safety ~*~
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

/// The type of an index into a board
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Index {
    pub row: Number,
    pub col: Number,
}

/// The actual type of a sudoku square, distinguishing between different cases naturally without
/// having to resort to icky bitwise shenanigans!
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Square {
    Unfilled,
    Fixed(Number),
    Guessed(Number),
}

/// The base type for a sudoku board. I *think* Rust should automatically lay out the doubly-nested
/// array in an efficient fashion?
pub struct Board {
    /// Bitsets of what numbers are in each row
    rows: [[bool; 9]; 9],
    /// Bitsets of what numbers are in each col
    cols: [[bool; 9]; 9],
    /// Bitsets of what numbers are in each square
    squares: [[bool; 9]; 9],
    /// The actual numbers on the board
    board: [[Square; 9]; 9],
}

impl Number {
    /// Simple way to iterate over all numbers
    pub fn iter() -> impl Iterator<Item = Self> {
        use Number::*;
        static NUMBERS: [Number; 9] = [One, Two, Three, Four, Five, Six, Seven, Eight, Nine];
        NUMBERS.iter().map(|x| *x)
    }

    /// Returns the number coming after this one, if there is any
    pub fn next(self) -> Option<Self> {
        use Number::*;
        match self {
            One => Some(Two),
            Two => Some(Three),
            Three => Some(Four),
            Four => Some(Five),
            Five => Some(Six),
            Six => Some(Seven),
            Seven => Some(Eight),
            Eight => Some(Nine),
            Nine => None,
        }
    }

    /// Return the underlying value of the number, for indexing purposes
    /// ENSURES: 0 <= \return < 9
    fn as_index(self) -> usize {
        use Number::*;
        match self {
            One => 0,
            Two => 1,
            Three => 2,
            Four => 3,
            Five => 4,
            Six => 5,
            Seven => 6,
            Eight => 7,
            Nine => 8,
        }
    }

    /// Constructs a Number from its ASCII representation. Returns None if the byte isn't a valid
    /// number
    pub fn from_byte(b: u8) -> Option<Self> {
        use Number::*;
        match b {
            b'1' => Some(One),
            b'2' => Some(Two),
            b'3' => Some(Three),
            b'4' => Some(Four),
            b'5' => Some(Five),
            b'6' => Some(Six),
            b'7' => Some(Seven),
            b'8' => Some(Eight),
            b'9' => Some(Nine),
            _ => None,
        }
    }
}

impl Index {
    /// Gets the next (row, col) index after this one. "After" is indices with larger row values,
    /// or larger col values in the same row.
    pub fn next(self) -> Option<Self> {
        match self.col.next() {
            Some(next_col) => Some(Index {
                row: self.row,
                col: next_col,
            }),
            None => Some(Index {
                row: self.row.next()?,
                col: Number::One,
            }),
        }
    }
}

impl Board {
    /// Attempts to construct a board from numbers in a array. If these numbers already have an
    /// inconsistency, returns None. Otherwise
    pub fn from_array(orig_board: [[Option<Number>; 9]; 9]) -> Option<Self> {
        let mut board = [[Square::Unfilled; 9]; 9];
        for r in 0..9 {
            for c in 0..9 {
                if let Some(n) = orig_board[r][c] {
                    board[r][c] = Square::Fixed(n);
                }
            }
        }

        Board {
            board,
            rows: [[false; 9]; 9],
            cols: [[false; 9]; 9],
            squares: [[false; 9]; 9],
        }
        .ensure_consistency()
    }

    /// See `Board::from_array`
    fn ensure_consistency(mut self) -> Option<Self> {
        for n in Number::iter() {
            self.rows[n.as_index()] = has_duplicates(self.row(n))?;
            self.cols[n.as_index()] = has_duplicates(self.col(n))?;
            self.squares[n.as_index()] = has_duplicates(self.square(n))?;
        }

        Some(self)
    }

    /// Given the number for a row, return an iterator over all items in that row
    pub fn row(&self, r: Number) -> impl Iterator<Item = Square> + '_ {
        self.board[r.as_index()].iter().map(|x| *x)
    }

    /// Given the number for a column, return an iterator over all items in that column
    pub fn col(&self, c: Number) -> impl Iterator<Item = Square> + '_ {
        self.board.iter().map(move |r| r[c.as_index()])
    }

    /// Given the number for a 3x3 square, indexed as follows:
    /// ```
    /// +---+---+---+
    /// | 1 | 2 | 3 |
    /// +---+---+---+
    /// | 4 | 5 | 6 |
    /// +---+---+---+
    /// | 7 | 8 | 9 |
    /// +---+---+---+
    /// ```
    /// return an iterator over all items in that square
    pub fn square(&self, s: Number) -> impl Iterator<Item = Square> + '_ {
        use Number::*;

        struct BoardSquareIterator<'a> {
            board: &'a Board,
            index: Option<Number>,
            row: Number,
            col: Number,
            starting_col: Number,
        }

        impl<'a> BoardSquareIterator<'a> {
            fn new(board: &'a Board, s: Number) -> Self {
                let (row, col) = match s {
                    One => (One, One),
                    Two => (One, Four),
                    Three => (One, Seven),
                    Four => (Four, One),
                    Five => (Four, Four),
                    Six => (Four, Seven),
                    Seven => (Seven, One),
                    Eight => (Seven, Four),
                    Nine => (Seven, Seven),
                };

                Self {
                    board,
                    index: Some(One),
                    row,
                    col,
                    starting_col: col,
                }
            }
        }

        impl Iterator for BoardSquareIterator<'_> {
            type Item = Square;

            fn next(&mut self) -> Option<Self::Item> {
                let index = self.index?;
                self.index = index.next();

                let out = self.board.board[self.row.as_index()][self.col.as_index()];

                match index {
                    Three | Six => {
                        assert_ne!(self.row, Nine);
                        self.row = self.row.next().unwrap();
                        self.col = self.starting_col;
                    }
                    Nine => {}
                    _ => {
                        assert_ne!(self.col, Nine);
                        self.col = self.col.next().unwrap();
                    }
                }

                Some(out)
            }
        }

        BoardSquareIterator::new(self, s)
    }

    /// Returns whether all squares on the board have been filled
    pub fn is_filled(&self) -> bool {
        for row in self.board.iter() {
            for col in row.iter() {
                if let Square::Unfilled = col {
                    return false;
                }
            }
        }

        true
    }

    /// Given a (row, col) index, attempt to set the corresponding value to a number. This returns
    /// true iff the number is placed successfully, and false if it cannot be placed.
    pub fn place(&mut self, i: Index, n: Number) -> bool {
        use Square::*;
        match self[i] {
            Fixed(_) => false,
            Guessed(_) | Unfilled => {
                let to_place = Guessed(n);
                let n = n.as_index();
                let row = &mut self.rows[i.row.as_index()];
                if row[n] {
                    return false;
                }
                let col = &mut self.cols[i.col.as_index()];
                if col[n] {
                    return false;
                }
                let square = &mut self.squares[index_to_square(i).as_index()];
                if square[n] {
                    return false;
                }
                // All checks have passed, OK to modify now
                self.board[i.row.as_index()][i.col.as_index()] = to_place;
                row[n] = true;
                col[n] = true;
                square[n] = true;
                true
            }
        }
    }
}

impl std::ops::Index<Index> for Board {
    type Output = Square;

    fn index(&self, index: Index) -> &Self::Output {
        &self.board[index.row.as_index()][index.col.as_index()]
    }
}

/// Given a row and column, return the square it lies in. See `Board::square`
fn index_to_square(i: Index) -> Number {
    use Number::*;
    match i.row {
        One | Two | Three => match i.col {
            One | Two | Three => One,
            Four | Five | Six => Two,
            Seven | Eight | Nine => Three,
        },
        Four | Five | Six => match i.col {
            One | Two | Three => Four,
            Four | Five | Six => Five,
            Seven | Eight | Nine => Six,
        },
        Seven | Eight | Nine => match i.col {
            One | Two | Three => Seven,
            Four | Five | Six => Eight,
            Seven | Eight | Nine => Nine,
        },
    }
}

/// Given an iterator over `Square`s, detect if there are any duplicate `Number`s in the `Fixed` and
/// `Guessed` variants. If there are duplicates, returns None, otherwise returns Some with a bitset
/// of the numbers it has seen.
fn has_duplicates(square_iterator: impl Iterator<Item = Square>) -> Option<[bool; 9]> {
    let mut seen = [false; 9];

    for square in square_iterator {
        use Square::*;
        match square {
            Unfilled => (),
            Fixed(number) | Guessed(number) => {
                if seen[number.as_index()] {
                    return None;
                } else {
                    seen[number.as_index()] = true;
                }
            }
        }
    }

    Some(seen)
}
