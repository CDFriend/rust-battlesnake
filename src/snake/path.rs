//
// Pathing utilities
//

use queues::*;
use std::rc::Rc;
use std::collections::HashSet;

use super::map::{Map, BoardSpace};

struct BfsNode {
    /// Distance from root node
    dist: u32,

    /// Pointer to previous node in path from root
    prev: Option<Rc<BfsNode>>,

    /// Coordinates
    x: u32,
    y: u32,
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
        let opt_target = bfs_to(&map, (2, 4), (0, 3));
        assert!(opt_target.is_some());
    }

}
