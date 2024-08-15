use core::fmt;
use std::ops::Sub;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Coordinates((usize, usize));

impl Coordinates {
    pub fn new(position: (usize, usize)) -> Self {
        Self(position)
    }

    pub fn distance_to(&self, other: &Coordinates) -> usize {
        let abs_position = self - other;
        let (y, x) = abs_position.0;
        y + x
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (y, x) = self.0;
        write!(f, "({},{})", y, x)
    }
}

impl<'a, 'b> Sub<&'b Coordinates> for &'a Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: &'b Coordinates) -> Self::Output {
        let (ly, lx) = self.0;
        let (ry, rx) = rhs.0;
        let y = ly.abs_diff(ry);
        let x = lx.abs_diff(rx);
        Coordinates::new((y, x))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pair(Coordinates, Coordinates);

impl Pair {
    pub fn new(a: Coordinates, b: Coordinates) -> Self {
        Self(a, b)
    }

    pub fn into_coordinates(self) -> (Coordinates, Coordinates) {
        (self.0, self.1)
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 || self.0 == other.1 && self.1 == other.0
    }
}

impl PartialEq<(&Coordinates, &Coordinates)> for Pair {
    fn eq(&self, other: &(&Coordinates, &Coordinates)) -> bool {
        let (&a, &b) = other;
        Self::new(a, b) == *self
    }
}

#[cfg(test)]
mod tests {
    use crate::position::Coordinates;

    use super::Pair;

    #[test]
    fn sub1() {
        let a = Coordinates::new((2, 5));
        let b = Coordinates::new((6, 4));
        assert_eq!(&a - &b, Coordinates::new((4, 1)));
    }

    #[test]
    fn sub2() {
        let a = Coordinates::new((123, 654));
        let b = Coordinates::new((56, 44));
        assert_eq!(&a - &b, Coordinates::new((67, 610)));
    }

    #[test]
    fn sub3() {
        let a = Coordinates::new((65, 10));
        let b = Coordinates::new((19, 90));
        assert_eq!(&a - &b, Coordinates::new((46, 80)));
    }

    #[test]
    fn distance1() {
        let a = Coordinates::new((5, 1));
        let b = Coordinates::new((0, 5));
        assert_eq!(a.distance_to(&b), 9);
    }

    #[test]
    fn distance2() {
        let a = Coordinates::new((9, 0));
        let b = Coordinates::new((4, 12));
        assert_eq!(a.distance_to(&b), 17);
    }

    #[test]
    fn pair_normal() {
        let a = Coordinates::new((1, 2));
        let b = Coordinates::new((4, 7));
        let p1 = Pair::new(a, b);
        let p2 = Pair::new(a, b);
        assert_eq!(p1, p2);
    }

    #[test]
    fn pair_crossed() {
        let a = Coordinates::new((1, 2));
        let b = Coordinates::new((4, 7));
        let p1 = Pair::new(a, b);
        let p2 = Pair::new(b, a);
        assert_eq!(p1, p2);
    }
}
