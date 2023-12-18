use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
    pub fn adjacent(&self) -> impl Iterator<Item = Point> + '_ {
        ADJACENT.into_iter().map(|offset| self.add(offset))
    }
}

impl Point {
    pub fn x_offset(&self, other: &Point) -> i64 {
        (self.x - other.x).abs()
    }
    pub fn y_offset(&self, other: &Point) -> i64 {
        (self.y - other.y).abs()
    }
    pub fn manhattan(&self, other: &Point) -> u32 {
        (self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y))
            as u32
    }
}

impl Add<Offset> for Point {
    type Output = Self;
    fn add(self, Offset { dx, dy }: Offset) -> Self {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

impl Sub for Point {
    type Output = Offset;
    fn sub(self, Point { x, y }: Point) -> Offset {
        Offset {
            dx: self.x - x,
            dy: self.y - y,
        }
    }
}

impl TryFrom<&str> for Point {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.split_once(',') {
            Some((x, y)) => match (x.parse::<i64>(), y.parse::<i64>()) {
                (Ok(x), Ok(y)) => Ok(Point { x, y }),
                _ => Err("Invalid input"),
            },
            _ => Err("Invalid input"),
        }
    }
}

impl Mul<i64> for Offset {
    type Output = Offset;
    fn mul(self, n: i64) -> Offset {
        Offset {
            dx: self.dx * n,
            dy: self.dy * n,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Offset {
    pub dx: i64,
    pub dy: i64,
}

pub const UP: Offset = Offset { dx: 0, dy: -1 };
pub const DOWN: Offset = Offset { dx: 0, dy: 1 };
pub const RIGHT: Offset = Offset { dx: 1, dy: 0 };
pub const LEFT: Offset = Offset { dx: -1, dy: 0 };

pub const ADJACENT: [Offset; 4] = [UP, RIGHT, DOWN, LEFT];

impl Offset {
    pub fn new(dx: i64, dy: i64) -> Offset {
        Offset { dx, dy }
    }
}

impl Add<Point> for Offset {
    type Output = Point;
    fn add(self, Point { x, y }: Point) -> Point {
        Point {
            x: x + self.dx,
            y: y + self.dy,
        }
    }
}

impl Add for Offset {
    type Output = Self;
    fn add(self, Offset { dx, dy }: Offset) -> Self {
        Offset {
            dx: dx + self.dx,
            dy: dy + self.dy,
        }
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &LEFT => write!(f, "L"),
            &RIGHT => write!(f, "R"),
            &UP => write!(f, "U"),
            &DOWN => write!(f, "D"),
            _ => write!(f, "[{},{}]", self.dx, self.dy),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let p1 = Point::new(1, 2);
        let o1 = Offset::new(3, 4);
        let p2 = p1 + o1;
        assert_eq!(p2, Point::new(4, 6));
        let o2 = p2 - p1;
        assert_eq!(o2, o1);
    }
    #[test]
    fn test_xoffset() {
        assert_eq!(Point::new(0, 0).x_offset(&Point::new(-5, 7)), 5);
    }
    #[test]
    fn test_yoffset() {
        assert_eq!(Point::new(0, 0).y_offset(&Point::new(-5, 7)), 7);
    }
    #[test]
    fn test_offset_mul() {
        assert_eq!(Offset::new(1, -1) * 5, Offset::new(5, -5));
    }
    #[test]
    fn test_manhattan() {
        assert_eq!(Point::new(0, 0).manhattan(&Point::new(-5, 7)), 12);
    }
    #[test]
    fn test_from() {
        assert_eq!(Point::try_from("-3,6"), Ok(Point::new(-3, 6)));
    }
    #[test]
    fn test_adjacent() {
        assert_eq!(
            Point::new(5, 5).adjacent().collect::<Vec<_>>(),
            vec![
                Point::new(5, 4),
                Point::new(6, 5),
                Point::new(5, 6),
                Point::new(4, 5)
            ]
        );
    }
}
