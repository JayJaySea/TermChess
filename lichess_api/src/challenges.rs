use crate::connection::Connection;
use serde::Deserialize;
use crate::game::Game;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AiChallengeResponse {
    id: String,
}

pub struct ChallengeCreator<'a> {
    connection: &'a Connection
}

impl<'a> ChallengeCreator<'a> {
    pub fn new(connection: &'a Connection) -> ChallengeCreator {
        ChallengeCreator {
            connection
        }
    }

    pub fn ai(&self) -> Game {
        let resp: AiChallengeResponse = self.connection.create_reqest("POST", "https://lichess.org/api/challenge/ai")
            .send_form(&[("level", "4")])
            .unwrap().into_json().unwrap();
        println!("{:?}", resp);

        Game::new(self.connection, resp.id)
    }
}
