use crate::snake::utils::TwoDimensionalMap;
use crate::snake::api::SnakeConfig;
use super::utils::Move;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BoardSpace {
    EMPTY,
    SNAKE,
    FOOD,
}

impl Default for BoardSpace {
    fn default() -> Self { BoardSpace::EMPTY }
}

// Macros for move types
macro_rules! left {
    ($coords:expr) => {
        ($coords.0 - 1, $coords.1)
    }
}
macro_rules! right {
    ($coords:expr) => {
        ($coords.0 + 1, $coords.1)
    }
}
macro_rules! up {
    ($coords:expr) => {
        ($coords.0, $coords.1 - 1)
    }
}
macro_rules! down {
    ($coords:expr) => {
        ($coords.0, $coords.1 + 1)
    }
}

pub struct Map {
    pub width: u32,
    pub height: u32,

    // FIXME: using a matrix representation will use a lot of memory on large
    // boards. Use some sort of std::map equivalent?
    vals: TwoDimensionalMap<BoardSpace>,

    /// Location of your snake's head.
    you_head: (u32, u32)
}

impl Map {

    pub fn new(config: &SnakeConfig) -> Map {

        let board = &config.board;

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
            you_head: (config.you.body[0].x, config.you.body[0].y)
        }
    }

    pub fn at(&self, x: u32, y: u32) -> BoardSpace {
        self.vals[(x as usize, y as usize)]
    }

    /// Whether or not moving a given direction is safe (not a snake and not out
    /// or bounds).
    pub fn is_safe_move(&self, move_req: Move) -> bool {
        let head = &self.you_head;

        match move_req {

            Move::Up => {
                head.1 != 0 && self.is_safe_node(up!(head))
            },

            Move::Down => {
                head.1 + 1 < self.height && self.is_safe_node(down!(head))
            },

            Move::Left => {
                head.0 != 0 && self.is_safe_node(left!(head))
            },

            Move::Right => {
                head.0 + 1 < self.width && self.is_safe_node(right!(head))
            }

        }
    }

    pub fn is_safe_node(&self, coords: (u32, u32)) -> bool {
        if coords.0 >= self.width {
            return false;
        }
    
        if coords.1 >= self.height {
            return false;
        }
    
        if self.at(coords.0, coords.1) == BoardSpace::SNAKE {
            return false;
        }
    
        true
    }

    /// Find any move that won't (immediately) kill you
    pub fn find_safe_move(&self) -> Move {
        // No point in returning an option here. If there's no safe moves then
        // admit defeat and go left.
        let mut move_val = Move::Left;

        if self.is_safe_move(Move::Right) {
            move_val = Move::Right;
        }
        else if self.is_safe_move(Move::Left) {
            move_val = Move::Left;
        }
        else if self.is_safe_move(Move::Up) {
            move_val = Move::Up;
        }
        else if self.is_safe_move(Move::Down) {
            move_val = Move::Down;
        }

        move_val
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::snake::api::*;

    #[test]
    fn populates_food_and_snakes() {
        // Construct dummy board
        let config = SnakeConfig {
            board: Board {
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
            },
            ..Default::default()
        };

        let map = Map::new(&config);
        let board = &config.board;

        // Food should be placed on the map at the correct location
        for coords in board.food.iter() {
            assert_eq!(map.at(coords.x, coords.y), BoardSpace::FOOD)
        }

        // Snake should be placed in correct location
        for snake in board.snakes.iter() {
            for coords in snake.body.iter() {
                assert_eq!(map.at(coords.x, coords.y), BoardSpace::SNAKE)
            }
        }
    }

}
