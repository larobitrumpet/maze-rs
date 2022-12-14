#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

use Direction::*;

impl Direction {
    pub fn bit(&self) -> u8 {
        match self {
            Up => 0,
            Right => 1,
            Down => 2,
            Left => 3,
        }
    }

    pub fn oposite(&self) -> Direction {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }

}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point {x, y}
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn point_in_direction(&self, dir: Direction, width: usize, height: usize) -> Result<Point, ()> {
        match dir {
            Up => {
                if self.y() == 0 {
                    Err(())
                } else {
                    Ok(Point::new(self.x(), self.y() - 1))
                }
            },
            Right => {
                if self.x() == width - 1 {
                    Err(())
                } else {
                    Ok(Point::new(self.x() + 1, self.y()))
                }
            },
            Down => {
                if self.y() == height - 1 {
                    Err(())
                } else {
                    Ok(Point::new(self.x(), self.y() + 1))
                }
            },
            Left => {
                if self.x() == 0 {
                    Err(())
                } else {
                    Ok(Point::new(self.x() - 1, self.y()))
                }
            },
        }
    }

    pub fn next(&self, width: usize, height: usize) -> Option<Point> {
        let x = self.x();
        let y = self.y();
        if x + 1 < width {
            Some(Self::new(x + 1, y))
        } else if y + 1 < height {
            Some(Self::new(0, y + 1))
        } else {
            None
        }
    }
}
