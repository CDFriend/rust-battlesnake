use std::ops::{Index, IndexMut};

use crate::snake::api::Board;

#[derive(PartialEq, Clone, Debug)]
pub enum BoardSpace {
    EMPTY,
    SNAKE,
    FOOD,
}

pub struct Map {
    width: u32,
    height: u32,

    // FIXME: using a matrix representation will use a lot of memory on large
    // boards. Use some sort of std::map equivalent?
    vals: Vec<BoardSpace>,
}

impl Index<(u32, u32)> for Map {

    type Output = BoardSpace;

    fn index(&self, coords: (u32, u32)) -> &BoardSpace {
        let x = coords.0;
        let y = coords.1;

        assert!(y * self.height + x < self.width * self.height);
        &self.vals[(y * self.height + x) as usize]
    }

}

impl IndexMut<(u32, u32)> for Map {

    fn index_mut(&mut self, coords: (u32, u32)) -> &mut BoardSpace {
        let x = coords.0;
        let y = coords.1;

        assert!(y * self.height + x < self.width * self.height);
        &mut self.vals[(y * self.height + x) as usize]
    }

}

impl Map {

    pub fn new(board: &Board) -> Map {

        let mut map = Map {
            width: board.width,
            height: board.height,
            vals: vec![BoardSpace::EMPTY; (board.width * board.height) as usize],
        };

        // Add food first, then snakes
        for coords in board.food.iter() {
            map[(coords.x, coords.y)] = BoardSpace::FOOD;
        }

        for snake in board.snakes.iter() {
            for coords in snake.body.iter() {
                map[(coords.x, coords.y)] = BoardSpace::SNAKE;
            }
        }

        map
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::snake::api::*;

    #[test]
    fn populates_food_and_snakes() {
        // Construct dummy board
        let board = Board {
            width: 25,
            height: 20,
            snakes: vec!(
                Snake {
                    body: vec!(
                        Coords { x: 1, y: 1 },
                        Coords { x: 1, y: 2 },
                        Coords { x: 2, y: 2 },
                    ),
                    ..Default::default()
                }
            ),
            food: vec!(
                Coords { x: 22, y: 18 },
                Coords { x: 5,  y: 2 }
            ),
        };

        let map = Map::new(&board);

        // Food should be placed on the map at the correct location
        for coords in board.food {
            assert_eq!(map[(coords.x, coords.y)], BoardSpace::FOOD)
        }

        // Snake should be placed in correct location
        for snake in board.snakes {
            for coords in snake.body {
                assert_eq!(map[(coords.x, coords.y)], BoardSpace::SNAKE)
            }
        }
    }

}
