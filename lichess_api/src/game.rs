use crate::connection::Connection;
use std::io::{BufRead, self};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GameStateType {
    #[serde(rename="type")]
    state_type: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FullGameState {
    state: PartialGameState    
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PartialGameState {
    moves: String,
    wtime: u64,
    btime: u64,
    status: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatLineGameState {
    username: String,
    text: String,
    room: String
}

pub enum GameState {
    Full(FullGameState),
    State(PartialGameState),
    ChatLine(ChatLineGameState),
    Err
}

pub struct Game<'a> {
    connection: &'a Connection,
    id: String
}

impl<'a> Game<'a> {
    pub fn new(connection: &'a Connection, id: String) -> Game {
        Game {
            connection, id
        }
    }

    pub fn stream(&self) -> impl Iterator<Item = GameState> {
        let resp = self.connection.create_reqest("GET", format!("https://lichess.org/api/board/game/stream/{}", self.id).as_str())
            .call().unwrap();
   
        io::BufReader::new(resp.into_reader()).lines()
            .take_while(|line| line.is_ok())
            .map(|line| line.unwrap())
            .filter(|line| line.len() > 1)
            .map(|line| -> (GameStateType, String) {(serde_json::from_str(&line).unwrap(), line)})
            .map(|(state, line)| match state.state_type.as_str() {
                "gameFull" => GameState::Full(serde_json::from_str(&line).unwrap()),
                "gameState" => GameState::State(serde_json::from_str(&line).unwrap()),
                "chatLine" => GameState::ChatLine(serde_json::from_str(&line).unwrap()),
                _ => todo!()
            })
    }

    pub fn abort(self) {
        self.connection.create_reqest("POST", format!("https://lichess.org/api/board/game/{}/abort", self.id).as_str())
            .call().unwrap();
    }

    pub fn resign(self) {
        self.connection.create_reqest("POST", format!("https://lichess.org/api/board/game/{}/resign", self.id).as_str())
            .call().unwrap();
    }

    
}
