use rand::seq::SliceRandom;

use crate::GRID_SIZE;

type TileValue = Option<u32>;
type GridArray = [[TileValue; GRID_SIZE]; GRID_SIZE];

pub struct Grid(GridArray);

impl Grid {
    pub fn new() -> Self {
        let mut grid = GridArray::default();

        // Get all tile values
        let mut values: Vec<TileValue> = (0..GRID_SIZE.pow(2) as u32)
            .map(|value| if value == 0 { None } else { Some(value) })
            .collect();

        // Shuffle tile values
        let mut rng = rand::thread_rng();
        values.shuffle(&mut rng);

        for row in &mut grid {
            for tile in row {
                *tile = values.pop().unwrap()
            }
        }

        Self(grid)
    }

    pub fn iter(&self) -> GridIterator {
        GridIterator {
            grid: self,
            row: 0,
            col: 0,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&TileValue> {
        self.0.get(y)?.get(x)
    }
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut TileValue> {
        self.0.get_mut(y)?.get_mut(x)
    }

    pub fn find_empty(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        // Check if starting tile exists on board, and is not empty
        if !self.get(x, y).is_some_and(|tile| tile.is_some()) {
            return None;
        }

        let (start_x, start_y) = (x as isize, y as isize);

        // Cardinal directions
        // Clockwise from top (top, right, bottom, left)
        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        // Search in spiral, clockwise outwards, up to length of full board
        for dist in 0..GRID_SIZE as isize {
            for (dx, dy) in directions {
                // Get new x,y values
                let (x, y) = (start_x + dx * dist, start_y + dy * dist);

                // Safely convert signed pairs to unsigned
                let Some((x, y)) = isize_to_usize_pairs(x, y) else {
                    continue;
                };

                // Tile must exist on board
                if let Some(tile) = self.get(x, y) {
                    // Tile must be empty
                    if tile.is_none() {
                        return Some((x as usize, y as usize));
                    }
                }
            }
        }

        None
    }

    pub fn shift_tiles(&mut self, x: usize, y: usize) {
        let Some((new_x, new_y)) = self.find_empty(x, y) else {
            return;
        };
        *self.get_mut(new_x, new_y).unwrap() = *self.get(x, y).unwrap();
        *self.get_mut(x, y).unwrap() = None;
    }

    pub fn is_complete(&self) -> bool {
        for GridTile { x, y, tile } in self.iter() {
            let expected_value = y * GRID_SIZE + x + 1;

            let expected_tile = if expected_value >= GRID_SIZE.pow(2) {
                None
            } else {
                Some(expected_value as u32)
            };

            if tile != &expected_tile {
                return false;
            }
        }
        true
    }
}

fn isize_to_usize_pairs(x: isize, y: isize) -> Option<(usize, usize)> {
    let x = x.try_into().ok()?;
    let y = y.try_into().ok()?;
    Some((x, y))
}

pub struct GridIterator<'a> {
    grid: &'a Grid,
    row: usize,
    col: usize,
}

pub struct GridTile<'a> {
    pub tile: &'a TileValue,
    pub x: usize,
    pub y: usize,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = GridTile<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < GRID_SIZE {
            let tile = &self.grid.0[self.row][self.col];
            let x = self.col;
            let y = self.row;

            self.col += 1;
            if self.col == GRID_SIZE {
                self.col = 0;
                self.row += 1;
            }

            Some(GridTile { tile, x, y })
        } else {
            None
        }
    }
}
