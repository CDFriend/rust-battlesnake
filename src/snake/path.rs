//
// Pathing utilities
//

use queues::*;
use std::rc::Rc;
use std::collections::HashSet;

use super::map::{Map, BoardSpace};

/// Potential moves a snake can make
#[derive(Debug, PartialEq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right
}

impl Move {

    pub fn to_string(&self) -> &'static str {
        match self {
            Move::Up => "up",
            Move::Down => "down",
            Move::Left => "left",
            Move::Right => "right"
        }
    }

}

struct BfsNode {
    /// Distance from root node
    dist: u32,

    /// Pointer to previous node in path from root
    prev: Option<Rc<BfsNode>>,

    /// Coordinates
    x: u32,
    y: u32,
}

/// Represents a node on a snake's path. Paths are represented as linked lists from source to 
/// destination.
pub struct PathNode {
    // Coordinates of current node (x, y)
    coords: (u32, u32),

    // Next move to take on the path, or None if path is complete
    pub next_move: Option<Move>,
}


/// Gets a path from the source node to the target node.
pub fn shortest_path_to(map: &Map, start: (u32, u32), target: (u32, u32)) -> Option<Vec<PathNode>> {

    // Run BFS - is there a path to the target?
    let mut cur_bfs_node = match bfs_to(map, start, target) {
        Some(rc_target) => rc_target,
        None => return None
    };

    // Follow path backwards until we reach the source node.
    // Total required space for the path should be equal to the distance of the path.
    let mut path = Vec::<PathNode>::with_capacity((cur_bfs_node.dist + 1) as usize);

    let mut idx = 0;
    loop {

        // Determine next move
        let mut next_move : Option<Move> = None;
        if idx > 0 {

            // Determine the required move from the current node to the next one in the path
            let next_node = &path[idx - 1];

            if cur_bfs_node.x > next_node.coords.0 {
                next_move = Some(Move::Left);
            }
            else if cur_bfs_node.x < next_node.coords.0 {
                next_move = Some(Move::Right);
            }
            else if cur_bfs_node.y > next_node.coords.1 {
                next_move = Some(Move::Up);
            }
            else if cur_bfs_node.y < next_node.coords.1 {
                next_move = Some(Move::Down);
            }
        }

        path.push(PathNode{
            coords: (cur_bfs_node.x, cur_bfs_node.y),
            next_move: next_move
        });

        match &cur_bfs_node.prev {
            Some(node_rc) => cur_bfs_node = node_rc.clone(),
            None => break
        };

        idx = idx + 1;
    }

    path.reverse();

    Some(path)
}


/// Performs a breadth-first search trying to find the shortest path from a source node (x,y) to
/// a target node (x,y).
/// 
/// If the target node is accessible from the start node, returns Some(BfsNode). Otherwise returns
/// None.
/// 
/// Panics if elements can't be added to the queue (e.g. due to a lack of memory resources).
fn bfs_to(map: &Map, start_coords: (u32, u32), target_coords: (u32, u32)) -> Option<Rc<BfsNode>> {

    let mut q : Queue<Rc<BfsNode>> = Queue::new();
    let mut traversed : HashSet<(u32, u32)> = HashSet::new();

    // Add start node to the queue
    q.add(Rc::new(BfsNode{
        dist: 0,
        prev: None,
        x: start_coords.0,
        y: start_coords.1,
    })).unwrap();

    while q.size() > 0 {

        let cur_node = q.remove().unwrap();

        let x = (*cur_node).x;
        let y = (*cur_node).y;

        // Have we seen this node before?
        if traversed.contains(&(x, y)) {
            continue;
        }

        // Now we have!
        traversed.insert((x, y));

        // Have we found our target node?
        if x == target_coords.0 && y == target_coords.1 {
            return Some(cur_node);
        }

        // Examine neighbors, and add to the queue if they are valid nodes

        if is_valid_node(map, (x + 1, y)) {
            // Right
            q.add(Rc::new(BfsNode{
                x: x + 1,
                y: y,
                dist: (*cur_node).dist + 1,
                prev: Some(Rc::clone(&cur_node))
            })).unwrap();
        }

        if x != 0 && is_valid_node(map, (x - 1, y)) {
            // Left
            q.add(Rc::new(BfsNode{
                x: x - 1,
                y: y,
                dist: (*cur_node).dist + 1,
                prev: Some(Rc::clone(&cur_node))
            })).unwrap();
        }

        if y != 0 && is_valid_node(map, (x, y - 1)) {
            // Up
            q.add(Rc::new(BfsNode{
                x: x,
                y: y - 1,
                dist: (*cur_node).dist + 1,
                prev: Some(Rc::clone(&cur_node))
            })).unwrap();
        }

        if is_valid_node(map, (x, y + 1)) {
            // Down
            q.add(Rc::new(BfsNode{
                x: x,
                y: y + 1,
                dist: (*cur_node).dist + 1,
                prev: Some(Rc::clone(&cur_node))
            })).unwrap();
        }
    }

    // If we reached the end of the loop, target node is inaccessible
    None
}

/// Returns true if the provided coordinates can be traversed on the map.
fn is_valid_node(map: &Map, coords: (u32, u32)) -> bool {
    if coords.0 >= map.width {
        return false;
    }

    if coords.1 >= map.height {
        return false;
    }

    if map.at(coords.0, coords.1) == BoardSpace::SNAKE {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::snake::api::{Board, Snake, Coords};

    #[test]
    fn bfs_finds_target_node() {
        // Board state:
        //   - H - - - -
        //   - S - - - -
        //   - S - - - -
        //   T S - - - -
        //   - - Y - - -
        //
        // We should be able to reach the target node (T) from our current
        // position (Y).

        let map = Map::new(&Board{
            width: 6,
            height: 5,
            food: vec![],
            snakes: vec!(
                Snake {
                    body: vec!(
                        Coords { x: 1, y: 0 },
                        Coords { x: 1, y: 1 },
                        Coords { x: 1, y: 2 },
                        Coords { x: 1, y: 3 },
                    ),
                    ..Default::default()
                }
            )
        });

        // Should be able to reach node
        let path = shortest_path_to(&map, (2, 4), (0, 3));
        assert!(path.is_some());

        // Path should be (2, 4), (1, 4), (0, 4), (0, 3)
        let path = path.unwrap();
        assert_eq!(path[0].coords, (2, 4));
        assert_eq!(path[0].next_move, Some(Move::Left));
        assert_eq!(path[1].coords, (1, 4));
        assert_eq!(path[1].next_move, Some(Move::Left));
        assert_eq!(path[2].coords, (0, 4));
        assert_eq!(path[2].next_move, Some(Move::Up));
        assert_eq!(path[3].coords, (0, 3));
        assert_eq!(path[3].next_move, None);
    }

    #[test]
    fn bfs_determines_target_node_inaccessible() {
        // Board state:
        //   - H - - - -
        //   - S - - - -
        //   - S - - - -
        //   T S - - - -
        //   - S Y - - -
        //
        // The BFS algorithm should determine that that target node is inaccessible
        // from the source.

        let map = Map::new(&Board{
            width: 6,
            height: 5,
            food: vec![],
            snakes: vec!(
                Snake {
                    body: vec!(
                        Coords { x: 1, y: 0 },
                        Coords { x: 1, y: 1 },
                        Coords { x: 1, y: 2 },
                        Coords { x: 1, y: 3 },
                        Coords { x: 1, y: 4 },
                    ),
                    ..Default::default()
                }
            )
        });

        // Should be able to reach node
        let path = shortest_path_to(&map, (2, 4), (0, 3));
        assert!(path.is_none());
    }

    #[test]
    fn bfs_handles_short_path() {
        // BFS algorithm should be able to handle a short (1-node) path

        let map = Map::new(&Board {
            width: 20,
            height: 20,
            food: vec![],
            snakes: vec![]
        });

        let path = shortest_path_to(&map, (0, 0), (0, 1));
        assert!(path.is_some());

        // Path should include source node and target node
        let path = path.unwrap();
        assert_eq!(path[0].coords, (0, 0));
        assert_eq!(path[0].next_move, Some(Move::Down));
        assert_eq!(path[1].coords, (0, 1));
        assert!(path[1].next_move.is_none());
    }

    #[test]
    fn bfs_handles_no_path() {
        // In the strange case where the source and target node are the
        // same, the BFS algorithm should return a path with only the source node.

        let map = Map::new(&Board {
            width: 20,
            height: 20,
            food: vec![],
            snakes: vec![]
        });

        let path = shortest_path_to(&map, (0, 0), (0, 0));
        assert!(path.is_some());

        // Path should include source node and target node
        let path = path.unwrap();
        assert_eq!(path[0].coords, (0, 0));
        assert!(path[0].next_move.is_none());
    }

}
