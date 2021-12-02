pub mod connection;
pub mod authorization;
pub mod challenges;
pub mod game;

use serde::Deserialize;
use std::io::{BufRead, self};
use std::fs;
use chess_api::movement::{Move, Square};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Account {
    username: String,
    online: bool
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Game {
    game_id: String,
    is_my_turn: bool
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Games {
    now_playing: Vec<Game>        
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GameStreamType {
    #[serde(rename="type")]
    stream_type: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GameState {
    moves: String,
    status: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FullGame {
    id: String,
    initial_fen: String,
    state: GameState
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ChatLine {
    username: String,
    text: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Challenge {
    id: String,
    status: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Challenges {
    #[serde(rename="in")]
    in_challenges: Vec<Challenge>,
    #[serde(rename="out")]
    out_challenges: Vec<Challenge>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ChallengeRequest {
    challenge: Challenge
}

pub struct Connection {
    authorization: String,
    game_id: Option<String>
}

impl Connection {
    pub fn new(token: &str) -> Connection {
        let authorization = format!("Bearer {}", token);

        Connection {
            authorization,
            game_id: None
        }
    }
    
    pub fn get_account(&self) {
       let resp = ureq::get("https://lichess.org/api/account")
           .set("Authorization", &self.authorization)
           .call()
           .unwrap();
        
        let json: Account = resp.into_json().unwrap();
        println!("{:?}", json);
    }

    pub fn get_games(&mut self) {
       let resp = ureq::get("https://lichess.org/api/account/playing")
           .set("Authorization", &self.authorization)
           .query("nb", "50")
           .call()
           .unwrap();

        let mut json: Games = resp.into_json().unwrap();
        println!("{:?}", json);
        
        if !json.now_playing.is_empty() {
            self.game_id = Some(json.now_playing.remove(0).game_id);
        }
    }

    pub fn stream_events() {
        todo!();
    }

    pub fn perform_move(&self, m: Move) {
        let game_id = self.game_id.as_ref().unwrap();

        match ureq::post(format!("https://lichess.org/api/board/game/{}/move/{}", game_id, m.to_uci()).as_str())
           .set("Authorization", &self.authorization)
           .call() {
            
            Ok(_) => (),
            Err(ureq::Error::Status(code, resp)) => panic!("{} => {}", code, resp.into_string().unwrap()),
            Err(_) => panic!("Unknown error")
        }
    }

    pub fn write_in_chat(&self, msg: &str) {
        let game_id = self.game_id.as_ref().unwrap();
        
        ureq::post(format!("https://lichess.org/api/board/game/{}/chat", game_id).as_str())
            .set("Authorization", &self.authorization)
            .send_form(&[
                ("room", "player"),
                ("text", msg)
            ]).unwrap();
    }

    pub fn get_challenges(&self) {
        let resp = ureq::get("https://lichess.org/api/challenge")
            .set("Authorization", &self.authorization).call().unwrap();

        let json: Challenges = resp.into_json().unwrap();
        println!("{:?}", json);
    }

    pub fn create_challenge(&self, username: &str) {
        let resp = ureq::post(format!("https://lichess.org/api/challenge/{}", username).as_str())
            .set("Authorization", &self.authorization).call().unwrap();
   
        let json: ChallengeRequest = resp.into_json().unwrap();
        println!("{:?}", json);
    }

    pub fn accept_challenge(&self, id: &str) {
        ureq::post(format!("https://lichess.org/api/challenge/{}/accept", id).as_str())
            .set("Authorization", &self.authorization).call().unwrap();
    }

    pub fn decline_challenge() {
        todo!();
    }

    pub fn cancel_challenge() {
        todo!();
    }

    pub fn open_challenge(&self) {
        ureq::post("https://lichess.org/api/challenge/open")
            .set("Authorization", &self.authorization)
            .send_form(&[
                ("name", "Test, DO NOT JOIN")
            ]).unwrap();
    }
}


pub fn test() {
    let token = fs::read_to_string("/home/krzysztof1222/.config/lichess_token").expect("Failed to read api token");
    let conn = connection::Connection::new(&token);

//    let challenge = conn.challenge();
//    let game = challenge.ai();

    /*
    for state_change in game.stream().take(3) {
        match state_change {
            game::GameState::Full(full) => println!("Full: {:?}", full),
            game::GameState::State(state) => println!("State: {:?}", state),
            game::GameState::ChatLine(line) => println!("Line: {:?}", line),
            game::GameState::Err => panic!()
        }
    }*/

//    game.abort();
//    game.resign();
}
