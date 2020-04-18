//
// Snake endpoint handlers.
// 

pub mod api;
mod map;

use api::*;

pub fn handle_start(config: SnakeConfig) -> StartResponse {
    StartResponse {
        color: "#FF0000",
        headType: "beluga",
        tailType: "block-bum",
    }
}

pub fn handle_move(config: SnakeConfig) -> MoveResponse {
    MoveResponse {
        r#move: "left",
        shout: "Shooooot!"
    }
}

pub fn handle_end(config: SnakeConfig) {

}
