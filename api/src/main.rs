#[macro_use]
extern crate rocket;

use rocket::response::status;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/texts")]
fn get_texts() -> &'static str {
    "GET TEXTS"
}

#[post("/texts")]
fn post_text() -> &'static str {
    "POST TEXTS"
}

#[get("/games")]
fn get_games() -> &'static str {
    "GET GAMES"
}

#[post("/games")]
fn post_game() -> &'static str {
    "POST GAMES"
}

#[delete("/games")]
fn delete_game() -> &'static str {
    "DELETE GAMES"
}

#[get("/games/<id>/players")]
fn get_game_players(id: i32) -> status::Accepted<String> {
    status::Accepted(Some(format!("GET GAME PLAYERS {}", id)))
}

#[post("/games/<id>/join")]
fn join_game(id: i32) -> status::Accepted<String> {
    status::Accepted(Some(format!("JOIN GAME {}", id)))
}

#[post("/players/<id>/update-progress")]
fn update_progress(id: i32) -> status::Accepted<String> {
    status::Accepted(Some(format!("UPDATE PROGRESS {}", id)))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            index,
            get_texts,
            post_text,
            get_games,
            post_game,
            delete_game,
            get_game_players,
            join_game,
            update_progress
        ],
    )
}
