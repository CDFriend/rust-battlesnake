// Structues for Battlesnake API version 2020.01
// See https://docs.battlesnake.com/snake-api

use serde::Serialize;
use serde::Deserialize;

//
// Game structures
//

#[derive(Deserialize)]
pub struct Game {
    pub id: String,
}

#[derive(Deserialize)]
pub struct Coords {
    pub x: u32,
    pub y: u32,
}

#[derive(Deserialize)]
pub struct Snake {
    pub id: String,
    pub name: String,
    pub health: u8,
    pub body: Vec<Coords>,
    pub shout: String,
}

#[derive(Deserialize)]
pub struct Board {
    pub height: u32,
    pub width: u32,
    pub food: Vec<Coords>,
    pub snakes: Vec<Snake>,
}

#[derive(Deserialize)]
pub struct SnakeConfig {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: Snake,
}

//
// Response types
//

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct StartResponse {
    pub color: &'static str,
    pub headType: &'static str,
    pub tailType: &'static str,
}

#[derive(Serialize)]
pub struct MoveResponse {
    pub r#move: &'static str,
    pub shout: &'static str,
}
