use crate::point::Direction;
use crate::point::Point;

enum CellInfo {
    Dir(Direction),
    Position,
    Special,
    Visited,
    _Unused,
}

impl CellInfo {
    fn bit(&self) -> u8 {
        match self {
            CellInfo::Dir(dir) => dir.bit(),
            CellInfo::Position => 4,
            CellInfo::Special => 5,
            CellInfo::Visited => 6,
            CellInfo::_Unused => 7,
        }
    }
}

#[derive(Debug)]
pub struct Maze {
    cells: Vec<u8>,
    width: usize,
    height: usize,
    pos: Option<Point>,
    update: Vec<Point>,
}

impl Maze {
    pub fn new(width: usize, height: usize, wall_adder: bool) -> Maze {
        let mut maze = Maze {
            cells: vec![0; width * height],
            width,
            height,
            pos: None,
            update: vec![],
        };

        if wall_adder {
            maze.cells[0] = 0b0000_0110;
            maze.cells[width - 1] = 0b0000_1100;
            maze.cells[(height - 1) * width] = 0b0000_0011;
            maze.cells[height * width - 1] = 0b0000_1001;
            for x in 1..(width - 1) {
                maze.cells[x] = 0b0000_1110;
                maze.cells[(height - 1) * width + x] = 0b0000_1011;
            }
            for y in 1..(height - 1) {
                maze.cells[y * width] = 0b0000_0111;
                for x in 1..(width - 1) {
                    maze.cells[y * width + x] = 0b0000_1111;
                }
                maze.cells[y * width + width - 1] = 0b0000_1101;
            }
        }

        maze
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn update_pop(&mut self) -> Option<Point> {
        self.update.pop()
    }

    fn update_point(&mut self, p: Point) -> () {
        self.update.push(p);
    }

    fn get_cell_index(&self, p: Point) -> usize {
        if p.x() >= self.width() {
            panic!("x = {} is larger than width = {}", p.x(), self.width());
        }
        if p.y() >= self.height() {
            panic!("y = {} is larger than height = {}", p.y(), self.height());
        }

        p.y() * self.width() + p.x()
    }

    fn get_cell(&self, p: Point) -> u8 {
        self.cells[self.get_cell_index(p)]
    }

    fn set_cell(&mut self, p: Point, val: u8) -> () {
        let cell = self.get_cell_index(p);
        self.cells[cell] = val;
    }

    fn set_bit(val: u8, bit: u8) -> u8 {
        val | 1 << bit
    }

    fn clear_bit(val: u8, bit: u8) -> u8 {
        val & !(1 << bit)
    }

    fn set_value_bit(&mut self, p: Point, bit: u8) -> () {
        if bit > 7 {
            panic!("Can't set bit {bit} for an 8-bit number.");
        }
        self.set_cell(p, Self::set_bit(self.get_cell(p), bit));
    }

    fn clear_value_bit(&mut self, p: Point, bit: u8) -> () {
        if bit > 7 {
            panic!("Can't clear bit {bit} for an 8-bit number.");
        }
        self.set_cell(p, Self::clear_bit(self.get_cell(p), bit));
    }

    fn set_cell_info(&mut self, p: Point, info: CellInfo) -> () {
        self.set_value_bit(p, info.bit());
        self.update_point(p);
    }

    fn clear_cell_info(&mut self, p: Point, info: CellInfo) -> () {
        self.clear_value_bit(p, info.bit());
        self.update_point(p);
    }

    fn set_open(&mut self, p: Point, dir: Direction) -> () {
        self.set_cell_info(p, CellInfo::Dir(dir));
    }

    fn set_close(&mut self, p: Point, dir: Direction) -> () {
        self.clear_cell_info(p, CellInfo::Dir(dir));
    }

    pub fn carve_passage(&mut self, dir: Direction) -> () {
        match self.pos {
            None => panic!("Can't carve passage from a None position."),
            Some(pos) => {
                self.set_open(pos, dir);
                self.set_open(pos.point_in_direction(dir, self.width(), self.height()).unwrap(), dir.oposite());
            },
        }
    }

    pub fn fill_passage(&mut self, dir: Direction) -> () {
        match self.pos {
            None => panic!("Can't fill passage from a None position."),
            Some(pos) => {
                self.set_close(pos, dir);
                self.set_close(pos.point_in_direction(dir, self.width(), self.height()).unwrap(), dir.oposite());
            }
        }
    }

    pub fn set_pos(&mut self, p: Option<Point>) -> () {
        if let Some(pos) = self.pos {
            self.clear_cell_info(pos, CellInfo::Position);
        }
        if let Some(p) = p {
            self.set_cell_info(p, CellInfo::Position);
        }
        self.pos = p;
    }

    pub fn set_special(&mut self, p: Point) -> () {
        self.set_cell_info(p, CellInfo::Special);
    }

    pub fn clear_special(&mut self, p: Point) -> () {
        self.clear_cell_info(p, CellInfo::Special);
    }

    pub fn set_visited(&mut self, p: Point) -> () {
        self.set_cell_info(p, CellInfo::Visited)
    }

    pub fn get_visited(&self, p: Point) -> bool {
        self.get_cell(p) & 0b0100_0000 > 0
    }

    pub fn cell_passage(&self, p: Point) -> u8 {
        self.get_cell(p) & 0b0000_1111
    }

    pub fn cell_type(&self, p: Point) -> u8 {
        let num = (self.get_cell(p) & 0b0011_0000) >> 4;
        if num % 2 == 0 {
            num
        } else {
            Self::clear_bit(num, 1)
        }
    }
}
