/// # Move's square struct
///
/// holds information about move's start or end
///
/// # example
///
/// ```
/// use chess_api::movement::Square;
///
/// let _s = Square::new(0, 0);
/// ```
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Square {
    x: u8,
    y: u8,
}

impl Square {
    /// # Square's constructor
    ///
    /// note: `x` and `y` are 0 based 
    ///
    /// ```
    /// use chess_api::movement::Square;
    ///
    /// let s = Square::new(0, 1);
    /// assert_eq!(s.to_uci(), "a2");
    /// ```
    pub fn new(x: u8, y: u8) -> Square {
        assert!(x < 8);
        assert!(y < 8);

        Square {
            x, y   
        }
    }

    pub fn from_index(i: usize) -> Square {
        assert!(i < 64);

        Square {
            x: (i % 8) as u8,
            y: (i / 8) as u8
        }
    }

    pub fn to_uci(&self) -> String {
        format!("{}{}", "abcdefgh".chars().nth(self.x.into()).unwrap(), self.y + 1)
    }

    pub fn to_index(&self) -> usize {
        (self.x + 8 * self.y).into()
    }

    pub fn to_coords(&self) -> (u8, u8) {
        (self.x, self.y)
    }
}

fn min_max<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if a < b { (a, b) } else { (b, a) }
}

#[derive(Copy, Clone)]
pub struct Move {
    start: Square,
    end: Square,
}

impl Move {
    pub fn new(start: Square, end: Square) -> Move {
        assert_ne!(start, end);

        Move {
            start, end
        }
    }

    pub fn to_uci(&self) -> String {
        let mut result = self.start.to_uci();
        result.push_str(&self.end.to_uci());
        result
    }

    pub fn min_max_x(&self) -> (u8, u8) {
        min_max(self.start.x, self.end.x)
    }

    pub fn min_max_y(&self) -> (u8, u8) {
        min_max(self.start.y, self.end.y)
    }

    pub fn min_max_x_y(&self) -> ((u8, u8), (u8, u8)) {
        (self.min_max_x(), self.min_max_y())
    }

    pub fn to_deltas(&self) -> (u8, u8) {
        let (min_x, max_x) = self.min_max_x();
        let (min_y, max_y) = self.min_max_y();
        (max_x - min_x, max_y - min_y)
    }

    pub fn to_squares(&self) -> (Square, Square) {
        (self.start, self.end)
    }

    pub fn to_coords(&self) -> ((u8, u8), (u8, u8)) {
        (self.start.to_coords(), self.end.to_coords())
    }

    pub fn start(&self) -> Square {
        self.start
    }

    pub fn end(&self) -> Square {
        self.end
    }
}

pub struct LineMovement {
    current: Square,
    end: Square,
    include_end: bool
}

impl LineMovement {
    fn new(start: Square, end: Square, include_end: bool) -> LineMovement {
        LineMovement {
            end, current: start,
            include_end
        }
    }

    pub fn from(m: Move) -> LineMovement {
        LineMovement::new(m.start(), m.end(), false)
    }
}

impl Iterator for LineMovement {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None
        }

        if self.current.x < self.end.x {
            self.current.x += 1;
        } else if self.current.x > self.end.x {
            self.current.x -= 1;
        }

        if self.current.y < self.end.y {
            self.current.y += 1;
        } else if self.current.y > self.end.y {
            self.current.y -= 1;
        }

        if self.current == self.end && !self.include_end {
            None
        } else {
            Some(self.current)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn line_movement() {
        let mut movement = LineMovement::new(Square::new(6, 6), Square::new(1, 1), false);

        assert_eq!(movement.next(), Some(Square::new(5, 5)));
        assert_eq!(movement.next(), Some(Square::new(4, 4)));
        assert_eq!(movement.next(), Some(Square::new(3, 3)));
        assert_eq!(movement.next(), Some(Square::new(2, 2)));
        assert_eq!(movement.next(), None);
        
        let mut movement = LineMovement::new(Square::new(1, 1), Square::new(6, 6), false);

        assert_eq!(movement.next(), Some(Square::new(2, 2)));
        assert_eq!(movement.next(), Some(Square::new(3, 3)));
        assert_eq!(movement.next(), Some(Square::new(4, 4)));
        assert_eq!(movement.next(), Some(Square::new(5, 5)));
        assert_eq!(movement.next(), None);

        let mut movement = LineMovement::new(Square::new(1, 6), Square::new(1, 1), false);

        assert_eq!(movement.next(), Some(Square::new(1, 5)));
        assert_eq!(movement.next(), Some(Square::new(1, 4)));
        assert_eq!(movement.next(), Some(Square::new(1, 3)));
        assert_eq!(movement.next(), Some(Square::new(1, 2)));
        assert_eq!(movement.next(), None);
        
        let mut movement = LineMovement::new(Square::new(1, 6), Square::new(6, 6), false);

        assert_eq!(movement.next(), Some(Square::new(2, 6)));
        assert_eq!(movement.next(), Some(Square::new(3, 6)));
        assert_eq!(movement.next(), Some(Square::new(4, 6)));
        assert_eq!(movement.next(), Some(Square::new(5, 6)));
        assert_eq!(movement.next(), None);
    }

    #[test]
    fn uci_format_test() {
        let s = Move::new(Square::new(0, 0), Square::new(7, 7));
        assert_eq!(s.to_uci(), "a1h8");

        let s = Move::new(Square::new(1, 2), Square::new(3, 4));
        assert_eq!(s.to_uci(), "b3d5");
    }

    #[test]
    #[should_panic]
    fn square_x_overflow() {
        let _s = Square::new(8, 0);
    }

    #[test]
    #[should_panic]
    fn square_y_overflow() {
        let _s = Square::new(0, 8);
    }
}
