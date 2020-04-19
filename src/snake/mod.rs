//
// Snake endpoint handlers.
// 

pub mod api;
mod map;
mod utils;
mod path;

use api::*;
use map::Map;
use path::Move;

pub fn handle_start(_config: SnakeConfig) -> StartResponse {
    StartResponse {
        color: "#FF0000",
        headType: "beluga",
        tailType: "block-bum",
    }
}

pub fn handle_move(config: SnakeConfig) -> MoveResponse {

    // Find nearest food
    let food = &config.board.food[0];
    let head = &config.you.body[0];

    let map = Map::new(&config.board);
    let path = path::shortest_path_to(&map, (head.x, head.y), (food.x, food.y));

    let mut move_str = "left";
    if path.is_some() {
        let path_src = &path.unwrap()[0];

        // If path has a next move
        if path_src.next_move.is_some() {
            move_str = match path_src.next_move.as_ref().unwrap() {
                Move::Left => "left",
                Move::Right => "right",
                Move::Up => "up",
                Move::Down => "down",
            };
        }
    }

    MoveResponse {
        r#move: move_str,
        shout: "Shooooot!"
    }
}

pub fn handle_end(_config: SnakeConfig) {

}
