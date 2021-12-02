use crate::challenges::ChallengeCreator;
use ureq::Request;

pub struct Connection {
    token: String
}

impl Connection {
    pub fn new(token: &str) -> Connection {
        Connection {
            token: format!("Bearer {}", token)
        }
    }

    pub fn create_reqest(&self, method: &str, path: &str) -> Request {
        ureq::request(method, path).set("Authorization", self.token.as_str())
    }

    pub fn challenge(&self) -> ChallengeCreator {
        ChallengeCreator::new(self)
    }
}
