super::selection!();

#[derive(Debug, Clone)]
// #[repr(transparent)]
struct CharMatrix(Vec<Vec<u8>>);

impl CharMatrix {
    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.0.get(x)?.get(y).copied()
    }
}

#[derive(Debug, Clone, Copy)]
struct Cursor {
    x: usize,
    y: usize,
    height: usize,
    width: usize,
    completed: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(isize)]
enum Direction {
    Decrease = -1,
    Constant = 0,
    Increase = 1,
}

impl Cursor {
    pub fn new(char_matrix: &CharMatrix) -> Self {
        Self {
            x: 0,
            y: 0,
            height: char_matrix.0.len(),
            width: char_matrix.0[0].len(),
            completed: false,
        }
    }

    pub fn check(&self, char_matrix: &CharMatrix) -> usize {
        #[allow(clippy::enum_glob_use)]
        use Direction::*;
        [
            self.check_in_direction(char_matrix, Decrease, Decrease),
            self.check_in_direction(char_matrix, Constant, Decrease),
            self.check_in_direction(char_matrix, Increase, Decrease),
            self.check_in_direction(char_matrix, Decrease, Constant),
            // Constant, Constant would go here, but that would be silly
            self.check_in_direction(char_matrix, Increase, Constant),
            self.check_in_direction(char_matrix, Decrease, Increase),
            self.check_in_direction(char_matrix, Constant, Increase),
            self.check_in_direction(char_matrix, Increase, Increase),
        ]
        .iter()
        .copied()
        .map(usize::from)
        .sum()
    }

    fn check_in_direction(
        &self,
        char_matrix: &CharMatrix,
        y_direction: Direction,
        x_direction: Direction,
    ) -> bool {
        let x_motion = x_direction as isize;
        let y_motion = y_direction as isize;
        let (x1, y1, x2, y2, x3, y3) = (
            self.x.wrapping_add_signed(x_motion),
            self.y.wrapping_add_signed(y_motion),
            self.x.wrapping_add_signed(x_motion * 2),
            self.y.wrapping_add_signed(y_motion * 2),
            self.x.wrapping_add_signed(x_motion * 3),
            self.y.wrapping_add_signed(y_motion * 3),
        );
        let string = &[
            char_matrix.get(self.x, self.y).unwrap_or(b'.'),
            char_matrix.get(x1, y1).unwrap_or(b'.'),
            char_matrix.get(x2, y2).unwrap_or(b'.'),
            char_matrix.get(x3, y3).unwrap_or(b'.'),
        ];
        string == b"XMAS"
    }

    pub fn next(&mut self) {
        if !self.completed {
            self.y += 1;
            if self.y == self.width {
                self.y = 0;
                self.x += 1;
                if self.x == self.height {
                    self.completed = true;
                }
            }
        }
    }

    pub fn check_then_next(&mut self, char_matrix: &CharMatrix) -> usize {
        let result = self.check(char_matrix);
        self.next();
        result
    }
}

pub fn part1(file: &str) -> usize {
    // this is probably a massive slow down, but at least it only happens once
    let char_matrix = CharMatrix(file.lines().map(|l| l.bytes().collect()).collect());
    let mut cursor = Cursor::new(&char_matrix);
    let mut found_count = 0;
    while !cursor.completed {
        found_count += cursor.check_then_next(&char_matrix);
    }
    found_count
}

pub fn part2(_file: &str) -> usize {
    todo!()
}
