use serde::Deserialize;
use std::io::{BufRead, self};
use std::fs;

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

pub struct Connection {
    authorization: String 
}

impl Connection {
    pub fn new(token: &str) -> Connection {
        let authorization = format!("Bearer {}", token);

        Connection {
            authorization
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

    pub fn get_games(&self) {
       let resp = ureq::get("https://lichess.org/api/account/playing")
           .set("Authorization", &self.authorization)
           .query("nb", "50")
           .call()
           .unwrap();

        let json: Games = resp.into_json().unwrap();
        println!("{:?}", json);
    }

    pub fn stream_state(&self) {
        let resp = ureq::get("https://lichess.org/api/board/game/stream/KBxUya07")
           .set("Authorization", &self.authorization)
           .call().unwrap();

        let reader = resp.into_reader();
        let lines = io::BufReader::new(reader).lines();

        for line in lines {
            match line {
                Ok(line) => {
                    if line.len() == 0 {
                        continue;
                    }

                    let state: GameStreamType = match serde_json::from_str(&line) {
                        Ok(state) => state,
                        Err(_) => break
                    };
                
                    match state.stream_type.as_str() {
                        "gameFull" => {
                            let state: FullGame = serde_json::from_str(&line).unwrap();
                            println!("{:?}", state);
                        },
                        "gameState" => {
                            let state: GameState = serde_json::from_str(&line).unwrap();
                            println!("{:?}", state);
                        },
                        "chatLine" => {
                            let state: ChatLine = serde_json::from_str(&line).unwrap();
                            println!("{:?}", state);
                        },
                        _ => panic!("Unknown state")
                    }
                },
                Err(_) => break
            }
        }
    }
}


pub fn test() {
    let token = fs::read_to_string("/home/krzysztof1222/.config/lichess_token").expect("Failed to read api token");
    let connection = Connection::new(&token);
    connection.get_account();
    connection.get_games();
    connection.stream_state();
}
