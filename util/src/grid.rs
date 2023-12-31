use crate::point::{Offset, Point, ADJACENT};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub start: Point,
    pub end: Point,
    pub size: Offset,
    pub data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn check_bounds(&self, p: &Point) -> bool {
        (p.x >= self.start.x) && (p.y >= self.start.y) && (p.x <= self.end.x) && (p.y <= self.end.y)
    }
    pub fn get(&self, p: &Point) -> Option<&T> {
        if self.check_bounds(p) {
            let offset = *p - self.start;
            self.data
                .get((offset.dx + offset.dy * self.size.dx) as usize)
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, p: &Point) -> Option<&mut T> {
        if self.check_bounds(p) {
            let offset = *p - self.start;
            self.data
                .get_mut((offset.dx + offset.dy * self.size.dx) as usize)
        } else {
            None
        }
    }
    pub fn set(&mut self, p: &Point, new: T) -> Result<(), ()> {
        if let Some(old) = self.get_mut(p) {
            *old = new;
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn adjacent(&self, p: &Point) -> Vec<Point> {
        self.offset(p, ADJACENT)
    }
    pub fn offset<O: AsRef<[Offset]>>(&self, p: &Point, offsets: O) -> Vec<Point> {
        offsets
            .as_ref()
            .iter()
            .map(|&o| *p + o)
            .filter(|p| self.check_bounds(p))
            .collect()
    }
    pub fn index_to_point(&self, i: usize) -> Point {
        self.start + Offset::new(i as i64 % self.size.dx, i as i64 / self.size.dx)
    }
}

impl<T> Grid<T>
where
    T: PartialEq + Eq,
{
    pub fn find(&self, t: &T) -> Vec<Point> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, d)| {
                if d == t {
                    Some(self.index_to_point(i))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
    pub fn inside(&self, p: &Point, walls: &Vec<T>) -> bool {
        if !self.check_bounds(p) || walls.contains(self.get(p).unwrap()) {
            return false;
        }
        let mut wall = false;
        let mut n_walls = 0;
        for x in 0..=p.x {
            let current = self.get(&Point::new(x, p.y)).unwrap();
            if walls.contains(current) {
                wall = true;
            } else {
                if wall {
                    n_walls += 1;
                    wall = false;
                }
            }
        }
        let walls_l = n_walls;
        wall = false;
        n_walls = 0;
        for x in p.x..=self.end.x {
            let current = self.get(&Point::new(x, p.y)).unwrap();
            if walls.contains(current) {
                wall = true;
            } else {
                if wall {
                    n_walls += 1;
                    wall = false;
                }
            }
        }
        let walls_r = n_walls;
        println!("{} : {}-{}", p, walls_l, walls_r);
        false
    }
}

impl<T> Grid<T>
where
    T: PartialEq + Eq + Clone,
{
    pub fn fill(&mut self, start: &Point, wall: &Vec<T>, fill: Option<&T>) -> Vec<Point> {
        assert!(self.check_bounds(&start));
        assert!(!wall.contains(self.get(&start).unwrap()));
        let mut q = vec![start.clone()];
        let mut visited: HashSet<Point> = HashSet::new();
        while let Some(p) = q.pop() {
            if let Some(f) = fill {
                self.set(&p, f.clone()).unwrap();
            }
            visited.insert(p.clone());
            for adj in self.adjacent(&p) {
                if !visited.contains(&adj) {
                    let v = self.get(&adj).unwrap();
                    if !(wall.contains(&v)) {
                        q.push(adj);
                    }
                }
            }
        }
        visited.into_iter().collect()
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(v: Vec<Vec<T>>) -> Self {
        assert!(v.len() > 0);
        let start = Point::new(0, 0);
        let end = Point::new(v[0].len() as i64 - 1, v.len() as i64 - 1);
        let size = (end - start) + Offset::new(1, 1);
        let mut data = Vec::with_capacity(((size.dx + 1) * (size.dy + 1)) as usize);
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

impl<T: Clone> Grid<T> {
    pub fn empty(start: &Point, end: &Point, value: T) -> Self {
        let size = (*end - *start) + Offset::new(1, 1);
        assert!(size.dx >= 0 && size.dy >= 0);
        let mut data = Vec::with_capacity((size.dx * size.dy) as usize);
        for _ in 0..(size.dx * size.dy) {
            data.push(value.clone());
        }
        Self {
            start: start.clone(),
            end: end.clone(),
            size,
            data,
        }
    }
    pub fn draw_line(&mut self, p1: &Point, p2: &Point, v: T) -> Result<(), ()> {
        match (p1.x == p2.x, p1.y == p2.y) {
            (true, _) => Ok(for y in min(p1.y, p2.y)..=max(p1.y, p2.y) {
                self.set(&Point::new(p1.x, y), v.clone())?;
            }),
            (_, true) => Ok(for x in min(p1.x, p2.x)..=max(p1.x, p2.x) {
                self.set(&Point::new(x, p1.y), v.clone())?;
            }),
            _ => Err(()),
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

    fn make_grid() -> Grid<char> {
        let mut g: Grid<char> = Grid::empty(&Point::new(-2, -2), &Point::new(2, 2), '.');
        let mut c = 'A';
        for y in -2..=2 {
            for x in -2..=2 {
                g.set(&Point::new(x, y), c).unwrap();
                c = char::from_u32(c as u32 + 1).unwrap();
            }
        }
        g
    }

    #[test]
    fn test_grid() {
        let mut g: Grid<char> = Grid::empty(&Point::new(-2, -2), &Point::new(2, 2), '.');
        g.set(&Point::new(0, 0), 'X').unwrap();
        assert_eq!(g.check_bounds(&Point::new(-1, -1)), true);
        assert_eq!(g.check_bounds(&Point::new(-5, 0)), false);
        assert_eq!(g.get(&Point::new(-3, -3)), None);
        assert_eq!(g.get(&Point::new(0, 0)), Some(&'X'));
        assert_eq!(g.get(&Point::new(-1, -1)), Some(&'.'));
    }

    #[test]
    fn test_grid_from() {
        let g = Grid::from(
            "ABCDE\nFGHIJ\nKLMNO\nPQRST\nUVWXY\n"
                .lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );
        assert_eq!(g.get(&Point::new(2, 2)), Some(&'M'));
        assert_eq!(g.get(&Point::new(4, 4)), Some(&'Y'));
    }

    #[test]
    fn test_grid_make() {
        let g = make_grid();
        assert_eq!(g.to_string(), "ABCDE\nFGHIJ\nKLMNO\nPQRST\nUVWXY\n");
    }

    #[test]
    fn test_grid_adjacent() {
        let g = make_grid();
        assert_eq!(
            g.adjacent(&Point::new(0, 0))
                .iter()
                .map(|p| g.get(p))
                .collect::<Option<Vec<_>>>(),
            Some(vec![&'H', &'N', &'R', &'L'])
        );
        assert_eq!(
            g.adjacent(&Point::new(2, -2))
                .iter()
                .map(|p| g.get(p))
                .collect::<Option<Vec<_>>>(),
            Some(vec![&'J', &'D'])
        );
    }

    #[test]
    fn test_grid_find() {
        let mut g = make_grid();
        assert_eq!(g.find(&'A'), vec![Point::new(-2, -2)]);
        assert_eq!(g.find(&'M'), vec![Point::new(0, 0)]);
        assert_eq!(g.find(&'Y'), vec![Point::new(2, 2)]);
        assert_eq!(g.find(&'Z'), vec![]);
        // Find multiple
        g.set(&Point::new(1, 1), 'A').unwrap();
        assert_eq!(g.find(&'A'), vec![Point::new(-2, -2), Point::new(1, 1)]);
    }

    #[test]
    fn test_grid_line() {
        let mut g: Grid<char> = Grid::empty(&Point::new(-2, -2), &Point::new(2, 2), '.');
        assert_eq!(
            g.draw_line(&Point::new(-2, 0), &Point::new(1, 0), '-'),
            Ok(())
        );
        assert_eq!(
            g.draw_line(&Point::new(2, -2), &Point::new(2, 2), '|'),
            Ok(())
        );
        assert_eq!(
            g.draw_line(&Point::new(-2, 0), &Point::new(1, -1), '-'),
            Err(())
        );
        assert_eq!(
            g.draw_line(&Point::new(-2, 0), &Point::new(5, 0), '-'),
            Err(())
        );
    }
    #[test]
    fn test_grid_fill() {
        let mut g = Grid::empty(&Point::new(0, 0), &Point::new(10, 10), '.');
        for p in vec![
            [1, 1],
            [3, 1],
            [3, 3],
            [7, 3],
            [7, 5],
            [9, 5],
            [9, 7],
            [3, 7],
            [3, 5],
            [1, 5],
            [1, 1],
        ]
        .windows(2)
        {
            g.draw_line(
                &Point::new(p[0][0], p[0][1]),
                &Point::new(p[1][0], p[1][1]),
                '#',
            )
            .unwrap();
        }
        assert_eq!(g.fill(&Point::new(2, 2), &vec!['#'], None).len(), 15);
        let mut p1 = g.fill(&Point::new(2, 2), &vec!['#'], Some(&'~'));
        let mut p2 = g.find(&'~');
        p1.sort();
        p2.sort();
        assert_eq!(p1, p2);
    }
    #[test]
    fn test_grid_inside() {
        let mut g = Grid::empty(&Point::new(0, 0), &Point::new(10, 10), '.');
        for p in vec![
            [1, 1],
            [3, 1],
            [3, 3],
            [7, 3],
            [7, 5],
            [9, 5],
            [9, 9],
            [7, 9],
            [7, 7],
            [5, 7],
            [5, 8],
            [3, 8],
            [3, 6],
            [1, 6],
            [1, 1],
        ]
        .windows(2)
        {
            g.draw_line(
                &Point::new(p[0][0], p[0][1]),
                &Point::new(p[1][0], p[1][1]),
                '#',
            )
            .unwrap();
        }
        println!("{}", g);
        for x in 0..=g.end.x {
            for y in 0..g.end.y {
                g.inside(&Point::new(x, y), &vec!['#']);
            }
        }
    }
}
