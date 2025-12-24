use crate::utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ManifoldCell {
    Empty,
    Source,
    Beam,
    Splitter,
}
type ManifoldMatrix  = Vec<Vec<ManifoldCell>>;

#[derive(Debug, Clone)]
struct Manifold {
    matrix: ManifoldMatrix,
    // spacetimes: Vec<ManifoldMatrix>,
}

impl Manifold {
    // pub fn new(x: usize, y: usize) -> Self {
    //     Self {
    //         cells: (0..y).map(|_| vec![ManifoldCell::Empty; x]).collect(),
    //     }
    // }

    pub fn get(&self, x: usize, y: usize) -> ManifoldCell {
        self.matrix[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: ManifoldCell) {
        self.matrix[y][x] = value;
    }

    pub fn from(input: &String) -> Self {
        let lines = utils::lines_to_vec(input);

        Self {
            matrix: lines
                .iter()
                .filter_map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => Some(ManifoldCell::Empty),
                            'S' => Some(ManifoldCell::Source),
                            '^' => Some(ManifoldCell::Splitter),
                            '|' => Some(ManifoldCell::Beam),
                            _ => None,
                        })
                        .collect()
                })
                .collect(),
        }
    }

    // fn fork(&self) -> Self{
    //     Self {
    //         matrix: self.matrix.iter().map(|r| r.clone()).collect()
    //     }
    // }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!();

        for row in self.matrix.iter() {
            for cell in row {
                match cell {
                    ManifoldCell::Empty => print!("."),
                    ManifoldCell::Source => print!("S"),
                    ManifoldCell::Splitter => print!("^"),
                    ManifoldCell::Beam => print!("|"),
                }
            }
            println!();
        }
        println!();
    }

    fn advance(&mut self, row_id: usize) {
        if row_id >= self.matrix.len() - 1 {
            return;
        }

        for x in 0..self.matrix[row_id].len() {
            let cell = self.get(x, row_id);
            let down_cell = self.get(x, row_id + 1);

            match cell {
                ManifoldCell::Source | ManifoldCell::Beam => {
                    if down_cell == ManifoldCell::Splitter {
                        self.set(x - 1, row_id + 1, ManifoldCell::Beam);
                        self.set(x + 1, row_id + 1, ManifoldCell::Beam);
                    } else {
                        self.set(x, row_id + 1, ManifoldCell::Beam);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn advance_all(&mut self) {
        for y in 0..self.matrix.len() {
            self.advance(y);
        }
    }

    pub fn count_splits(&self) -> u32 {
        let mut splits: u32 = 0;
        for (y, row) in self.matrix.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    ManifoldCell::Splitter => {
                        if self.matrix[y-1][x] == ManifoldCell::Beam {
                        splits += 1;

                        }
                    },
                    _=> {}
                }
            }
        }
        splits
    }
}

pub fn solve(input: &String) -> (String, String) {
    let mut m = Manifold::from(&input);

    // m.print();
    m.advance_all();
    // m.print();

    let splits = m.count_splits();

    // println!("Splits: {}", splits);

    (splits.to_string(), "0".to_string())
}
