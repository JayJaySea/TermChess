use crate::connection::Connection;

use serde::Deserialize;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use tiny_http::{Server, Response};

use sha2::{Sha256, Digest};
use url::Url;


enum HttpServerError {
    CouldNotCreateServer,
    CouldNotReceive
}

#[derive(Deserialize, Debug)]
struct Token {
    token_type: String,
    access_token: String,
    expires_in: u64
}

fn create_random_string(len: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(len).map(char::from).collect()
}

fn hash_string(string: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(string);
    hasher.finalize().to_vec()
}

fn get_verification_url(client_id: &str, code_challenge: &str, redirect_url: &str) -> String {
    format!("https://lichess.org/oauth?response_type=code&client_id={}&redirect_uri={}&code_challenge_method=S256&code_challenge={}&scope=challenge:read%20challenge:write%20board:play",
        client_id, redirect_url, code_challenge
    )
}

fn get_temporary_code() -> Result<String, HttpServerError> {
    let server = match Server::http("0.0.0.0:8080") {
        Ok(server) => server,
        Err(_) => return Err(HttpServerError::CouldNotCreateServer)
    };

    let code = loop {
        let request = match server.recv() {
            Ok(req) => req,
            Err(_) => return Err(HttpServerError::CouldNotReceive)
        };

        let url = match Url::parse("http://localhost:8080/").unwrap().join(request.url()) {
            Ok(url) => url,
            Err(_) => continue
        };

        match url.query_pairs().find_map(|(name, data)| if name == "code" { Some(data) } else { None }) {
            Some(code) => {
                request.respond(Response::empty(200)).unwrap();
                break String::from(code);
            },
            None => request.respond(Response::empty(401)).unwrap()
        }
    };

    Ok(code)
}

fn get_access_token(code: &str, code_verifier: &str, redirect_url: &str, client_id: &str) -> Token {
    ureq::post("https://lichess.org/api/token")
        .send_form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("code_verifier", code_verifier),
            ("redirect_uri", redirect_url), 
            ("client_id", client_id)
        ]).unwrap().into_json().unwrap()
}

impl Connection {
    pub fn authorize() -> Connection {
        let redirect_url = "http://localhost:8080/";
        let client_id = "term-chess";

        let code_verifier = create_random_string(64); 
        let code_challenge = hash_string(&code_verifier);
        let code_challenge = base64::encode_config(code_challenge, base64::URL_SAFE_NO_PAD);

        let verification_url = get_verification_url(client_id, &code_challenge, redirect_url);
        println!("Open in browser: {}", verification_url);

        let code = match get_temporary_code() { Ok(code) => code, Err(_) => todo!("Return http server error") };
        let token = get_access_token(&code, &code_verifier, redirect_url, client_id);

        Connection::new(&token.access_token)
    }
}
