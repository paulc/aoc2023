use crate::point::{Offset, Point, ADJACENT};
use std::cmp::{max, min};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    start: Point,
    end: Point,
    size: Offset,
    data: Vec<T>,
}

impl<T: Default> Grid<T> {
    pub fn new(start: Point, end: Point) -> Self {
        let size = (end - start) + Offset::new(1, 1);
        assert!(size.dx >= 0 && size.dy >= 0);
        let mut data = Vec::with_capacity((size.dx * size.dy) as usize);
        for _ in 0..(size.dx * size.dy) {
            data.push(Default::default());
        }
        Self {
            start,
            end,
            size,
            data,
        }
    }
    pub fn check_bounds(&self, p: Point) -> bool {
        (p.x >= self.start.x) && (p.y >= self.start.y) && (p.x <= self.end.x) && (p.y <= self.end.y)
    }
    pub fn get(&self, p: Point) -> Option<&T> {
        if self.check_bounds(p) {
            let offset = p - self.start;
            self.data
                .get((offset.dx + offset.dy * self.size.dx) as usize)
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, p: Point) -> Option<&mut T> {
        if self.check_bounds(p) {
            let offset = p - self.start;
            self.data
                .get_mut((offset.dx + offset.dy * self.size.dx) as usize)
        } else {
            None
        }
    }
    pub fn set(&mut self, p: Point, new: T) -> Result<(), ()> {
        if let Some(old) = self.get_mut(p) {
            *old = new;
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn adjacent(&self, p: Point) -> Vec<Point> {
        self.offset(p, ADJACENT)
    }
    pub fn offset<O: AsRef<[Offset]>>(&self, p: Point, offsets: O) -> Vec<Point> {
        offsets
            .as_ref()
            .iter()
            .map(|&o| p + o)
            .filter(|&p| self.check_bounds(p))
            .collect()
    }
}

impl<T: Clone + Default> Grid<T> {
    pub fn draw_line(&mut self, p1: Point, p2: Point, v: T) -> Result<(), ()> {
        match (p1.x == p2.x, p1.y == p2.y) {
            (true, _) => Ok(for y in min(p1.y, p2.y)..=max(p1.y, p2.y) {
                self.set(Point::new(p1.x, y), v.clone())?;
            }),
            (_, true) => Ok(for x in min(p1.x, p2.x)..=max(p1.x, p2.x) {
                self.set(Point::new(x, p1.y), v.clone())?;
            }),
            _ => Err(()),
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(v: Vec<Vec<T>>) -> Self {
        assert!(v.len() > 0);
        let start = Point::new(0, 0);
        let end = Point::new(v[0].len() as i32, v.len() as i32);
        let size = end - start;
        let mut data = Vec::with_capacity((size.dx * size.dy) as usize);
        for row in v {
            data.extend(row.into_iter());
        }
        Self {
            start,
            end,
            size,
            data,
        }
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "{}-{}", self.start, self.end)?;
        for y in 0..self.size.dy {
            for x in 0..self.size.dx {
                write!(f, "{}", self.data[(x + y * self.size.dx) as usize])?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Eq, PartialEq)]
    struct V(char);
    impl Default for V {
        fn default() -> Self {
            V('.')
        }
    }
    impl Display for V {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    fn fill_grid() -> Grid<V> {
        let mut g: Grid<V> = Grid::new(Point::new(-2, -2), Point::new(2, 2));
        let mut c = 'A';
        for y in -2..=2 {
            for x in -2..=2 {
                g.set(Point::new(x, y), V(c)).unwrap();
                c = char::from_u32(c as u32 + 1).unwrap();
            }
        }
        g
    }

    #[test]
    fn test_grid() {
        let mut g: Grid<V> = Grid::new(Point::new(-2, -2), Point::new(2, 2));
        g.set(Point::new(0, 0), V('X')).unwrap();
        assert_eq!(g.check_bounds(Point::new(-1, -1)), true);
        assert_eq!(g.check_bounds(Point::new(-5, 0)), false);
        assert_eq!(g.get(Point::new(-3, -3)), None);
        assert_eq!(g.get(Point::new(0, 0)), Some(&V('X')));
        assert_eq!(g.get(Point::new(-1, -1)), Some(&V('.')));
    }

    #[test]
    fn test_grid_from() {
        let g = Grid::from(
            "ABCDE\nFGHIJ\nKLMNO\nPQRST\nUVWXY\n"
                .lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );
        assert_eq!(g.get(Point::new(2, 2)), Some(&'M'));
        assert_eq!(g.get(Point::new(4, 4)), Some(&'Y'));
    }

    #[test]
    fn test_grid_fill() {
        let g = fill_grid();
        assert_eq!(g.to_string(), "ABCDE\nFGHIJ\nKLMNO\nPQRST\nUVWXY\n");
    }

    #[test]
    fn test_grid_adjacent() {
        let g = fill_grid();
        assert_eq!(
            g.adjacent(Point::new(0, 0))
                .into_iter()
                .map(|p| g.get(p))
                .collect::<Option<Vec<_>>>(),
            Some(vec![&V('H'), &V('N'), &V('R'), &V('L')])
        );
        assert_eq!(
            g.adjacent(Point::new(2, -2))
                .into_iter()
                .map(|p| g.get(p))
                .collect::<Option<Vec<_>>>(),
            Some(vec![&V('J'), &V('D')])
        );
    }

    #[test]
    fn test_grid_line() {
        let mut g: Grid<V> = Grid::new(Point::new(-2, -2), Point::new(2, 2));
        assert_eq!(
            g.draw_line(Point::new(-2, 0), Point::new(1, 0), V('-')),
            Ok(())
        );
        assert_eq!(
            g.draw_line(Point::new(2, -2), Point::new(2, 2), V('|')),
            Ok(())
        );
        assert_eq!(
            g.draw_line(Point::new(-2, 0), Point::new(1, -1), V('-')),
            Err(())
        );
        assert_eq!(
            g.draw_line(Point::new(-2, 0), Point::new(5, 0), V('-')),
            Err(())
        );
    }
}
