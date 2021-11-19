use crate::Move;
use crate::Square;

pub struct LineMovement {
    current: Square,
    start: Square,
    end: Square
}

impl LineMovement {
    pub fn new(start: Square, end: Square) -> LineMovement {
        LineMovement {
            start, end,
            current: start
        }
    }
}

impl Iterator for LineMovement {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
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

        if self.current == self.end {
            None
        } else {
            Some(self.current)
        }
    }
}

// todo move Move and Square here

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn line_movement() {
        let mut movement = LineMovement::new(Square::new(6, 6), Square::new(1, 1));

        assert_eq!(movement.next(), Some(Square::new(5, 5)));
        assert_eq!(movement.next(), Some(Square::new(4, 4)));
        assert_eq!(movement.next(), Some(Square::new(3, 3)));
        assert_eq!(movement.next(), Some(Square::new(2, 2)));
        assert_eq!(movement.next(), None);
        
        let mut movement = LineMovement::new(Square::new(1, 1), Square::new(6, 6));

        assert_eq!(movement.next(), Some(Square::new(2, 2)));
        assert_eq!(movement.next(), Some(Square::new(3, 3)));
        assert_eq!(movement.next(), Some(Square::new(4, 4)));
        assert_eq!(movement.next(), Some(Square::new(5, 5)));
        assert_eq!(movement.next(), None);

        let mut movement = LineMovement::new(Square::new(1, 6), Square::new(1, 1));

        assert_eq!(movement.next(), Some(Square::new(1, 5)));
        assert_eq!(movement.next(), Some(Square::new(1, 4)));
        assert_eq!(movement.next(), Some(Square::new(1, 3)));
        assert_eq!(movement.next(), Some(Square::new(1, 2)));
        assert_eq!(movement.next(), None);
        
        let mut movement = LineMovement::new(Square::new(1, 6), Square::new(6, 6));

        assert_eq!(movement.next(), Some(Square::new(2, 6)));
        assert_eq!(movement.next(), Some(Square::new(3, 6)));
        assert_eq!(movement.next(), Some(Square::new(4, 6)));
        assert_eq!(movement.next(), Some(Square::new(5, 6)));
        assert_eq!(movement.next(), None);
    }
}
