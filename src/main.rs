use rouille::Response;
use rouille::Request;
use rouille::try_or_400;

mod snake;

use snake::api::SnakeConfig;
use snake::{handle_start, handle_move, handle_end};

fn handle_request(request: &Request) -> Response {

    let snake_config: SnakeConfig = try_or_400!(rouille::input::json_input(request));

    match request.url().as_str() {
        "/start" => Response::json(&handle_start(snake_config)),
        "/move" =>  Response::json(&handle_move(snake_config)),
        "/end" =>   Response::json(&handle_end(snake_config)),
        _ => Response::empty_404(),
    }
}

fn main() {
    rouille::start_server_with_pool("0.0.0.0:8080", None, move|request| {
        handle_request(request)
    })
}
