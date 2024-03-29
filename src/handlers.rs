extern crate chrono;

use chrono::prelude::*;
// use rocket::http::ContentType;
// use rocket::request::Request;
// use rocket::response::{self, Responder, Response};
use rocket::serde::json::Json;
use serde::Serialize;
// use serde_json;
// use std::io::{Cursor, Read};
use std::time::Duration;

#[derive(Serialize)]
pub struct Task {
    id: u32,
    name: &'static str,
    message: String,
}

#[get("/")]
pub fn index() -> &'static str {
    std::thread::sleep(Duration::from_millis(100));
    "Hello, world!"
}

#[get("/json")]
pub fn json() -> Json<Task> {
    Json(Task {
        id: 12,
        name: "Coucou ceci est mon nom",
        message: String::from("Mon message doit être un peu long pour augmenter la taille"),
    })
}

#[get("/big-json")]
pub fn big_json() -> Json<Vec<Task>> {
    let mut v: Vec<Task> = Vec::new();
    for i in 1..=100_000 {
        v.push(Task {
            id: i,
            name: "Coucou ceci est mon nom",
            message: String::from("Mon message doit être un peu long pour augmenter la taille"),
        });
    }
    Json(v)
}

#[get("/utc-time")]
pub fn time_now() -> String {
    let now: DateTime<Utc> = Utc::now();

    now.to_rfc3339_opts(SecondsFormat::Secs, true)
}

/*
#[derive(Serialize, Debug)]
pub struct User {
    pub id: u32,
    pub lastname: String,
    pub firstname: String,
}

#[derive(Debug)]
pub struct UsersStream {
    pub state: State,
    pub users: Vec<User>,
    pub pos: usize,
    pub pending: Cursor<Vec<u8>>,
}

#[derive(Debug)]
pub enum State {
    Header,
    Users,
    Trailer,
    Done,
}

impl Read for UsersStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        loop {
            // first, try to read any unfinished data from the buffer
            match self.pending.read(buf) {
                // end of buffer; need to get more data
                Ok(0) => (),
                Ok(n) => return Ok(n),
                Err(e) => return Err(e),
            };

            // determine the next data to read
            match self.state {
                State::Header => {
                    self.pending = Cursor::new(vec![b'[']);
                    self.state = State::Users;
                }
                State::Users => {
                    // encode the next user
                    match self.users.get(self.pos) {
                        Some(user) => {
                            let mut bytes = vec![b','];
                            bytes.append(&mut serde_json::to_vec(user).unwrap());
                            self.pos += 1;
                            self.pending = Cursor::new(bytes);
                        }
                        None => self.state = State::Trailer,
                    }
                }
                State::Trailer => {
                    self.pending = Cursor::new(vec![b']']);
                    self.state = State::Done;
                }
                State::Done => return Ok(0),
            }
        }
    }
}

impl<'r> Responder<'r> for UsersStream {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .streamed_body(self)
            .header(ContentType::new("application", "json"))
            .ok()
    }
}

#[get("/big-json-stream")]
pub fn big_json_stream() -> Result<UsersStream, ()> {
    let mut v: Vec<User> = Vec::new();
    for i in 0..100_000 {
        v.push(User {
            id: i,
            lastname: "My lastname".to_owned(),
            firstname: String::from("My firstname"),
        });
    }
    Ok(UsersStream {
        state: State::Header,
        users: v,
        pos: 0,
        pending: Cursor::new(vec![]),
    })
}
*/

#[cfg(test)]
mod test {
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Hello, world!".into()));
    }
}
