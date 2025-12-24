use crate::utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifoldCell {
    Empty,
    Source,
    Beam,
    Splitter,
}
type ManifoldMatrix = Vec<Vec<ManifoldCell>>;

#[derive(Debug, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
pub struct Manifold {
    matrix: ManifoldMatrix,
    splits: Vec<Pos>,
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
            splits: vec![],
        }
    }

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

    fn advance(&mut self) {
        for y in 0..self.matrix.len() - 1 {
            for x in 0..self.matrix[y].len() {
                let cell = self.get(x, y);
                let down_cell = self.get(x, y + 1);

                match cell {
                    ManifoldCell::Source | ManifoldCell::Beam => {
                        if down_cell == ManifoldCell::Splitter {
                            self.set(x - 1, y + 1, ManifoldCell::Beam);
                            self.set(x + 1, y + 1, ManifoldCell::Beam);
                            self.splits.push(Pos { x: x, y: y + 1 });
                        } else {
                            self.set(x, y + 1, ManifoldCell::Beam);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn count_paths(&self) -> u64 {
        let rowed_splits: Vec<Vec<usize>> = (0..self.matrix.len())
            .map(|y| self.splits.iter().filter(|s| s.y == y).map(|s| s.x).collect())
            .collect();

        let start_x = self.matrix[0]
            .iter()
            .position(|&r| r == ManifoldCell::Source)
            .unwrap();

        let mut row_paths: Vec<u64> = vec![0; self.matrix[0].len()];
        row_paths[start_x] = 1;

        for splits in rowed_splits {
            for split_x in splits {
                row_paths[split_x - 1] += row_paths[split_x];
                row_paths[split_x + 1] += row_paths[split_x];
                row_paths[split_x] = 0;
            }
        }

        row_paths.iter().sum()
    }
}

pub fn solve(input: &String) -> (String, String) {
    let mut m = Manifold::from(&input);

    // m.print();
    m.advance();
    // m.print();

    let n_worlds = m.count_paths();

    (m.splits.len().to_string(), n_worlds.to_string())
}
