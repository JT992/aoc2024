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
#[repr(isize)]
enum Direction {
    Decrease = -1,
    Constant = 0,
    Increase = 1,
}

trait Cursor {
    fn next(&mut self);

    fn check(&self, char_matrix: &CharMatrix) -> usize;

    fn check_then_next(&mut self, char_matrix: &CharMatrix) -> usize {
        let result = self.check(char_matrix);
        self.next();
        result
    }
}

macro_rules! basic_cursor(
    {$cursor_name:ident} => {
        #[derive(Debug, Clone, Copy)]
        struct $cursor_name {
            x: usize,
            y: usize,
            height: usize,
            width: usize,
            completed: bool,
        }

        impl $cursor_name {
            pub fn new(char_matrix: &CharMatrix) -> Self {
                Self {
                    x: 0,
                    y: 0,
                    height: char_matrix.0.len(),
                    width: char_matrix.0[0].len(),
                    completed: false,
                }
            }
        }
    }
);

macro_rules! fn_next(
    () => {
        fn next(&mut self) {
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
    }
);

basic_cursor!(Part1Cursor);
basic_cursor!(Part2Cursor);

impl Cursor for Part1Cursor {
    fn check(&self, char_matrix: &CharMatrix) -> usize {
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

    fn_next!();
}

impl Part1Cursor {
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
}

impl Cursor for Part2Cursor {
    fn check(&self, char_matrix: &CharMatrix) -> usize {
        // first off, we're only checking the centre of these Xs,
        // so if we're not on an A there's just no point
        if char_matrix.get(self.x, self.y).unwrap_or(b'.') != b'A' {
            return 0;
        }
        // note to future self thinking these variable names are bad:
        // no
        // (left, right, top, bottom)
        let (xl, xr, yt, yb) = (
            self.x.wrapping_sub(1),
            self.x.wrapping_add(1),
            self.y.wrapping_sub(1),
            self.y.wrapping_add(1),
        );
        let right_descending = &[
            char_matrix.get(xl, yt).unwrap_or(b'.'),
            char_matrix.get(xr, yb).unwrap_or(b'.'),
        ];
        let left_descending = &[
            char_matrix.get(xr, yt).unwrap_or(b'.'),
            char_matrix.get(xl, yb).unwrap_or(b'.'),
        ];
        // we can just agree to ignore the centre character because it has to be A
        usize::from(
            (right_descending == b"MS" || right_descending == b"SM")
                && (left_descending == b"MS" || left_descending == b"SM"),
        )
    }

    fn_next!();
}

macro_rules! main_func(
    {$func_name:ident, $cursor_type:ty} => {
        pub fn $func_name(file: &str) -> usize {
            // this is probably a massive slow down, but at least it only happens once
            let char_matrix = CharMatrix(file.lines().map(|l| l.bytes().collect()).collect());
            let mut cursor = <$cursor_type>::new(&char_matrix);
            let mut found_count = 0;
            while !cursor.completed {
                found_count += cursor.check_then_next(&char_matrix);
            }
            found_count
        }
    }
);

main_func!(part1, Part1Cursor);
main_func!(part2, Part2Cursor);
