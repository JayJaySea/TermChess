mod board;
mod pieces;
mod movement;

pub fn min_max_rev<T: PartialOrd>(a: T, b: T) -> (T, T, bool) {
    if a < b { (a, b, false) } else { (b, a, true ) }
}

/// # Move's square struct
///
/// holds information about move's start or end
///
/// # example
///
/// ```
/// use chess_api::Square;
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
    /// use chess_api::Square;
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

    pub fn to_uci(&self) -> String {
        format!("{}{}", "abcdefgh".chars().nth(self.x.into()).unwrap(), self.y + 1)
    }

    pub fn to_index(&self) -> usize {
        (self.x + 8 * self.y).into()
    }
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
}












#[cfg(test)]
mod test {
    use super::*;

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
