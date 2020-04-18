use crate::snake::utils::TwoDimensionalMap;
use crate::snake::api::Board;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BoardSpace {
    EMPTY,
    SNAKE,
    FOOD,
}

impl Default for BoardSpace {
    fn default() -> Self { BoardSpace::EMPTY }
}

pub struct Map {
    width: u32,
    height: u32,

    // FIXME: using a matrix representation will use a lot of memory on large
    // boards. Use some sort of std::map equivalent?
    vals: TwoDimensionalMap<BoardSpace>,
}

impl Map {

    pub fn new(board: &Board) -> Map {

        let mut vals = TwoDimensionalMap::new(board.width as usize, board.height as usize);

        // Add food first, then snakes
        for coords in board.food.iter() {
            vals[(coords.x as usize, coords.y as usize)] = BoardSpace::FOOD;
        }

        for snake in board.snakes.iter() {
            for coords in snake.body.iter() {
                vals[(coords.x as usize, coords.y as usize)] = BoardSpace::SNAKE;
            }
        }

        Map {
            width: board.width,
            height: board.height,
            vals: vals,
        }
    }

    pub fn at(&self, x: u32, y: u32) -> BoardSpace {
        self.vals[(x as usize, y as usize)]
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
            assert_eq!(map.at(coords.x, coords.y), BoardSpace::FOOD)
        }

        // Snake should be placed in correct location
        for snake in board.snakes {
            for coords in snake.body {
                assert_eq!(map.at(coords.x, coords.y), BoardSpace::SNAKE)
            }
        }
    }

}
