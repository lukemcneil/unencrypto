#![feature(proc_macro_hygiene, decl_macro)]

#[cfg(test)]
mod tests;
mod types;
mod word_lookup;

use parking_lot::Mutex;
use rocket::http::Method;
use rocket::{
    self,
    config::{Environment, LoggingLevel},
    delete, get, post, put, routes, Config, State,
};
use rocket_contrib::{json::Json, serve::StaticFiles};
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::path::PathBuf;
use structopt::StructOpt;
use types::{CluesData, Game, GuessData, PlayerData, Result, Score};
use word_lookup::WordLookup;

type Games = Mutex<types::Games>;
type Words = Mutex<WordLookup>;

#[get("/heartbeat")]
fn heartbeat() -> &'static str {
    "heartbeat"
}

#[put("/game/<game_id>", data = "<player>")]
fn create_game(
    game_id: String,
    player: Json<PlayerData>,
    games: State<Games>,
    words: State<Words>,
) -> Result<()> {
    let mut games = games.lock();
    let player = player.into_inner();
    games.create(
        game_id,
        player.player,
        player.team,
        player.role,
        &mut words.lock(),
    )
}

#[post("/game/<game_id>", data = "<player>")]
fn join_game(game_id: String, player: Json<PlayerData>, games: State<Games>) -> Result<()> {
    let mut games = games.lock();
    let game = games.get(&game_id)?;
    let player = player.into_inner();
    game.add_player(player.player, player.team, player.role);
    Ok(())
}

#[get("/game/<game_id>")]
fn game(game_id: String, games: State<Games>) -> Result<Json<Game>> {
    let mut games = games.lock();
    let game = games.get(&game_id)?;
    // TODO: This clone is ugly
    Ok(Json(game.clone()))
}

#[post("/game/<game_id>/clues", data = "<clues>")]
fn clues(game_id: String, clues: Json<CluesData>, games: State<Games>) -> Result<()> {
    let mut games = games.lock();
    let game = games.get(&game_id)?;
    let clues = clues.into_inner();
    game.give_clues(clues.clues, clues.player.team)?;
    Ok(())
}

#[post("/game/<game_id>/guess_own_team", data = "<guess>")]
fn guess_own_team(game_id: String, guess: Json<GuessData>, games: State<Games>) -> Result<()> {
    let mut games = games.lock();
    let game = games.get(&game_id)?;
    let guess = guess.into_inner();
    game.guess_own_team(guess.guess, guess.player.team)?;
    Ok(())
}

#[post("/game/<game_id>/guess_other_team", data = "<guess>")]
fn guess_other_team(game_id: String, guess: Json<GuessData>, games: State<Games>) -> Result<()> {
    let mut games = games.lock();
    let game = games.get(&game_id)?;
    let guess = guess.into_inner();
    game.guess_other_team(guess.guess, guess.player.team)?;
    game.add_round_if_complete();
    Ok(())
}

#[delete("/game/<game_id>/exit", data = "<player>")]
fn exit_game(game_id: String, player: Json<PlayerData>, games: State<Games>) -> Result<()> {
    let mut games = games.lock();
    let game = games.get(&game_id)?;
    let player = player.into_inner();
    game.remove_player(player.player)
}

#[delete("/game/<game_id>")]
fn delete_game(game_id: String, games: State<Games>) -> Result<()> {
    let mut games = games.lock();
    games.delete(&game_id);
    Ok(())
}

#[get("/game/<game_id>/score")]
fn get_score(game_id: String, games: State<Games>) -> Result<Json<Score>> {
    let mut games = games.lock();
    let game = games.get(&game_id)?.clone();
    Ok(Json(game.get_score()))
}

fn rocket(opt: Option<Opt>) -> rocket::Rocket {
    let mut words = WordLookup::default();
    let rocket = if let Some(opt) = opt {
        if let Some(words_file) = opt.words_file {
            // Populate the words
            if let Err(e) = words.populate_from_file(&words_file) {
                eprintln!("Failed to populate words from file {words_file:?}, err: {e}");
                std::process::exit(1);
            }
        }
        // Configure the Rocket instance
        let config = Config::build(Environment::Staging)
            .address(opt.host)
            .port(opt.port)
            .log_level(opt.log_level)
            .finalize()
            .expect("to build Rocket Config");
        rocket::custom(config)
    } else {
        rocket::ignite()
    };
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Put]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::ignite().attach(cors.to_cors().unwrap());
    rocket
        .attach(cors.to_cors().unwrap())
        .mount("/", StaticFiles::from("../client-svelte/build/index.html"))
        .mount(
            "/api/v1",
            routes![
                heartbeat,
                create_game,
                join_game,
                game,
                clues,
                guess_own_team,
                guess_other_team,
                exit_game,
                delete_game,
                get_score,
            ],
        )
        .manage(Mutex::new(words))
        .manage(Games::default())
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// The path to a file containing newline delimited words.
    #[structopt(long = "words-file")]
    words_file: Option<PathBuf>,
    /// An IP address or host the application will listen on.
    #[structopt(long = "host", short = "H", default_value = "0.0.0.0")]
    host: String,
    /// A port number to listen on.
    #[structopt(long = "port", short = "P", default_value = "8172")]
    port: u16,
    /// The log level.
    #[structopt(
        default_value = "normal",
        long = "log-level",
        possible_values = &["off", "debug", "normal", "critical"]
    )]
    log_level: LoggingLevel,
}

fn main() {
    let opt = Opt::from_args();
    rocket(Some(opt)).launch();
}
