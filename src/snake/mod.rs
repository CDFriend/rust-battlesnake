//
// Snake endpoint handlers.
// 

pub mod api;

use api::*;

pub fn handle_start(config: SnakeConfig) -> StartResponse {
    StartResponse {
        color: String::from("#FF0000"),
        headType: String::from("beluga"),
        tailType: String::from("block-bum"),
    }
}

pub fn handle_move(config: SnakeConfig) -> MoveResponse {
    MoveResponse {
        r#move: String::from("left"),
        shout: String::from("Shooooot!")
    }
}

pub fn handle_end(config: SnakeConfig) {

}
