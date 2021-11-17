pub struct Square {
    x: u8,
    y: u8,
}

impl Square {
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
}

pub struct Move {
    start: Square,
    end: Square,
}

impl Move {
    pub fn new(start: Square, end: Square) -> Move {
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
