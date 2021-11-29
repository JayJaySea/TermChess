use ureq::Request;

use serde::Deserialize;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use tiny_http::{Server, Response};

use sha2::{Sha256, Sha512, Digest};
use url::{Url, ParseError};


#[derive(Deserialize, Debug)]
struct Token {
    token_type: String,
    access_token: String,
    expires_in: u64
}


pub struct Connection {
    token: String
}

impl Connection {
    pub fn new(token: &str) -> Connection {
        Connection {
            token: format!("Bearer {}", token)
        }
    }

    pub fn authorize() -> Connection {
        let code_verifier: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64).map(char::from)
            .collect();

        let mut hasher = Sha256::new();
        hasher.update(&code_verifier);
        let hashed_code_verifier = hasher.finalize();

        let hashed_code_verifier = base64::encode_config(hashed_code_verifier, base64::URL_SAFE_NO_PAD);

        let url = format!("https://lichess.org/oauth?response_type=code&client_id={}&redirect_uri={}&code_challenge_method=S256&code_challenge={}&scope=challenge:read%20challenge:write%20board:play",
                "term-chess",
                "http%3A%2F%2Flocalhost%3A8080",
                &hashed_code_verifier
            );

        println!("code_verifier = {}", &code_verifier);
        println!("hashed_code_verifier = {}", &hashed_code_verifier);
        println!("{}", url);

        let server = Server::http("0.0.0.0:8080").unwrap();

        let token = loop {
            let req = match server.recv() {
                Ok(req) => req,
                Err(e) => panic!("{:?}", e)
            };

            let req_url = req.url();
            let this_document = Url::parse("http://localhost:8080/").unwrap();
            let url = this_document.join(req_url).unwrap();
            let code = url.query_pairs().find_map(|(a, b)| if a == "code" { Some(b) } else { None });
            
            if let Some(code) = code {
                req.respond(Response::empty(200)).unwrap();
                break String::from(code);
            } else {
                req.respond(Response::empty(401)).unwrap();
            }
        };

        let resp = ureq::post("https://lichess.org/api/token")
            .send_form(&[
                ("grant_type", "authorization_code"),
                ("code", &token),
                ("code_verifier", &code_verifier),
                ("redirect_uri", "http://localhost:8080"), 
                ("client_id", "term-chess")
            ])
            .unwrap();

        let token: Token = resp.into_json().unwrap();

        Connection::new(&token.access_token)
    }

    pub fn create_reqest(&self, method: &str, path: &str) -> Request {
        ureq::request(method, path).set("Authorization", self.token.as_str())
    }
}
