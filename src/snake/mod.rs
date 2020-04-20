//
// Snake endpoint handlers.
// 

pub mod api;
mod map;
mod utils;
mod path;

use api::*;
use map::Map;
use utils::Move;
use path::shortest_path_to;

pub fn handle_start(_config: SnakeConfig) -> StartResponse {
    StartResponse {
        color: "#FF0000",
        headType: "beluga",
        tailType: "hook",
    }
}

pub fn handle_move(mut config: SnakeConfig) -> MoveResponse {

    // Remove duplicates from body. This might happen at the beginning of the game,
    // where we're sent 3 of the same sets of coordinates.
    config.you.body.dedup();

    // Chase your tail!
    let head = &config.you.body[0];
    let tail = &config.you.body[config.you.body.len() - 1];

    let map = Map::new(&config);
    
    let body = &config.you.body;
    let mut move_val = Move::Left;
    if body.len() < 3 {
        // Special case where head and tail are the same node (should only be
        // first move). Just try and find a direction that won't kill you.
        move_val = map.find_safe_move();
    }
    else {
        // Try to find your tail
        move_val = match shortest_path_to(&map, (head.x, head.y), (tail.x, tail.y)) {
            Some(path) => {
                // We've already checked that the head and tail are not the same node,
                // so we should have more than one node in our path
                assert!(path[0].next_move.is_some());
                path[0].next_move.unwrap_or(Move::Left)
            },
            None => {
                // No way to find your tail, so just go somewhere safe
                map.find_safe_move()
            }
        }
    }

    MoveResponse {
        r#move: move_val.to_string(),
        shout: "Shooooot!"
    }
}

pub fn handle_end(_config: SnakeConfig) {

}
