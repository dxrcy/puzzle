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
                *tile = values
                    .pop()
                    .expect("grid size is larger than amount of generated values")
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

    fn get(&self, (x, y): (usize, usize)) -> Option<&TileValue> {
        self.0.get(y)?.get(x)
    }
    fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut TileValue> {
        self.0.get_mut(y)?.get_mut(x)
    }

    pub fn find_empty(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        // Check if starting tile exists on board, and is not empty
        if !self.get((x, y)).is_some_and(|tile| tile.is_some()) {
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
                if let Some(tile) = self.get((x, y)) {
                    // Tile must be empty
                    if tile.is_none() {
                        return Some((x, y));
                    }
                }
            }
        }

        None
    }

    pub fn shift_tiles(&mut self, x: usize, y: usize) {
        let Some((nx, ny)) = self.find_empty(x, y) else {
            return;
        };

        // Case for each direction: Clockwise from top (UP, RIGHT, DOWN, LEFT)
        //     order of if-else chain should not matter, but is used as fallback for invalid game
        //     state
        // Create a vector tuples, which describe how tiles move:
        //     0: position of tile to move value from
        //     1: position of tile to move value to
        // Order of list is important, because tile value is changed on the next step of loop
        // Iterator is reversed for RIGHT and DOWN, because tiles move in negative order
        // Example shown in 'right' direction
        let tiles: Vec<_> = if ny < y {
            // UP
            (ny..y).map(|y| ((x, y + 1), (x, y))).collect()
        } else if nx > x {
            // RIGHT
            // For every position `x` (going right) from old position (leftmost) to
            //     new position (rightmost), create a 'move', which is the tile position, and
            //     the tile to the right
            // Reversed, so that loop goes right to left
            (x..nx).map(|x| ((x, y), (x + 1, y))).rev().collect()
        } else if ny > y {
            // DOWN
            (y..ny).map(|y| ((x, y), (x, y + 1))).rev().collect()
        } else {
            // LEFT
            (nx..x).map(|x| ((x + 1, y), (x, y))).collect()
        };

        // Move tiles as described above
        for (pos_future_value, pos_tile) in tiles {
            let future_value = *self.get(pos_future_value).expect("tile should exist");
            let tile = self.get_mut(pos_tile).expect("tile should exist");
            *tile = future_value;
        }
        // Tile which was clicked, should be removed (same as swapping with (nx,ny))
        *self.get_mut((x, y)).expect("tile should exist") = None;
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
