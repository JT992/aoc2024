super::selection!();

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Free = b'.',
    Visited = b'X',
    Obstacle = b'#',
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // hoping that the compiler can make this nice!
        let char = match self {
            Tile::Free => '.',
            Tile::Visited => 'X',
            Tile::Obstacle => '#',
        };
        write!(f, "{char}")
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)] // "unused" variants are created through transmutation
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl std::ops::Add<u8> for Direction {
    type Output = Direction;

    fn add(self, rhs: u8) -> Self::Output {
        // SAFETY: for all u8, u8 % 4 is between 0 and 3
        unsafe { std::mem::transmute((self as u8 + rhs) % 4) }
    }
}

impl Direction {
    pub fn turn(self) -> Self {
        self + 1
    }
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    y: usize,
    x: usize,
    direction: Direction,
}

impl Guard {
    fn is_in_bounds(&self, map: &Map) -> bool {
        map.is_in_bounds(self.y, self.x)
    }

    fn coords_ahead(&self) -> (usize, usize) {
        match self.direction {
            Direction::Up => (self.y.wrapping_sub(1), self.x),
            Direction::Down => (self.y + 1, self.x),
            Direction::Right => (self.y, self.x + 1),
            Direction::Left => (self.y, self.x.wrapping_sub(1)),
        }
    }

    fn tile_ahead(&self, map: &Map) -> Tile {
        let (y, x) = self.coords_ahead();
        map.get(y, x)
    }

    fn turn(&mut self) {
        self.direction = self.direction.turn();
    }

    fn advance(&mut self) {
        let (y, x) = self.coords_ahead();
        self.y = y;
        self.x = x;
    }

    /// Returns the number of visited tiles. This might be less than or equal to the number of
    /// steps if some tiles are "visited" more than once.
    pub fn patrol(&mut self, map: &mut Map) -> usize {
        // the first tile has already been visited
        let mut visited_count = 1;
        while self.is_in_bounds(map) {
            visited_count += usize::from(map.visit(self.y, self.x));
            while self.tile_ahead(map) == Tile::Obstacle {
                self.turn();
            }
            self.advance();
        }
        visited_count
    }
}

#[derive(Debug, Clone)]
struct Map {
    inner: Vec<Vec<Tile>>,
    height: usize,
    length: usize,
}

impl Map {
    pub fn is_in_bounds(&self, y: usize, x: usize) -> bool {
        y < self.height && x < self.length
    }

    pub fn get(&self, y: usize, x: usize) -> Tile {
        self.inner
            .get(y)
            .and_then(|r| r.get(x))
            .copied()
            .unwrap_or(Tile::Free)
        // we consider out of bounds to be Free because we want to move into it
    }

    /// Returns `true` if this tile has not yet been visited.
    ///
    /// Panics if this tile cannot be visited.
    pub fn visit(&mut self, y: usize, x: usize) -> bool {
        let tile = self
            .inner
            .get_mut(y)
            .and_then(|r| r.get_mut(x))
            .expect("attempted to visit an out-of-bounds tile");
        assert_ne!(
            tile,
            &Tile::Obstacle,
            "attempted to visit an obstacle located at {x},{y}"
        );
        std::mem::replace(tile, Tile::Visited) == Tile::Free
    }

    #[allow(dead_code)] // for checking that it works (it does)
    pub fn print(&self) {
        for line in &self.inner {
            for tile in line {
                print!("{tile}");
            }
            println!();
        }
    }
}

fn build_map(file: &str) -> (Map, Guard) {
    let mut guard = None;
    let mut outer = Vec::with_capacity(file.lines().count());
    let mut inner = Vec::new();
    for (y, row) in file.lines().enumerate() {
        // not sure if the default value from mem::take will have the right capacity
        // but if it does, then this is a no-op anyway
        inner.reserve(file.chars().take_while(|c| c != &'\n').count());
        for (x, char) in row.chars().enumerate() {
            let tile = match char {
                '^' => {
                    assert!(
                        guard.is_none(),
                        "there should only be one guard, found at least two"
                    );
                    guard = Some(Guard {
                        y,
                        x,
                        direction: Direction::Up,
                    });
                    Tile::Visited
                }
                '.' => Tile::Free,
                '#' => Tile::Obstacle,
                _ => panic!("invalid character {char} discovered"),
            };
            inner.push(tile);
        }
        // we use take to save an unnecessary clone
        outer.push(std::mem::take(&mut inner));
    }
    let height = outer.len();
    let length = outer[0].len();
    (
        Map {
            inner: outer,
            height,
            length,
        },
        guard.unwrap(),
    )
}

pub fn part1(file: &str) -> usize {
    let (mut map, mut guard) = build_map(file);
    guard.patrol(&mut map)
}

pub fn part2(_file: &str) -> usize {
    todo!()
}
