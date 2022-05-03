#[macro_use]
extern crate rocket;

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

// r.GET("/games/:id/players", handlers.GetGamePlayers(db))
// r.POST("/games/:id/join", handlers.JoinGame(db))
//
// r.POST("/players/:id/update-progress", handlers.UpdatePlayerProgress(db))

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
            delete_game
        ],
    )
}
