#[macro_use] extern crate rocket;
mod analysis;
mod seqdatastruct;
mod notes;
mod tuning;
mod wav;
use std::io::Result;

use analysis::{Chunk, analyze_chunk};
use rocket::data::{ToByteUnit};
use rocket::serde::json::Json;
use rocket::tokio::io::AsyncReadExt;
use rocket::{post, data::Data};
use serde::Serialize;


#[derive(Responder)]
#[response(status = 418, content_type = "json")]
struct Response(&'static str);

#[get("/")]
fn index() -> Response {
    Response("Hello, world!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, receive_wav_data])
}

// receive the data from the http request body
#[post("/wav_data", data = "<data>")]
async fn receive_wav_data(data: Data<'_>) -> Result<Json<Chunk>> {
    // read the base64 encoded data into a buffer
    let mut buffer = Vec::new();
    data.open(1.mebibytes()).read_to_end(&mut buffer).await?;

    // bad request if buffer is empty
    if buffer.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "empty buffer"));
    }

    // analyze the buffer
    let chunk = analyze_chunk(&buffer);
    Ok(Json(chunk))
}

// add unit test to test the function receive_wav_data
#[cfg(test)]
mod tests {
    use crate::notes::PhiNote;
    use crate::notes::Pitch;
    use crate::wav::Oscilator;
    use crate::wav::generate_wav;

    use super::*;
    use rocket::local::asynchronous::Client;

    #[rocket::async_test]
    async fn test_receive_wav_data() {
        let client = Client::tracked(rocket()).await.unwrap();

        // generate a signal and write it to a buffer
        let sine = generate_wav(&PhiNote {
            pitch: Pitch::from_str("A4").unwrap(),
            start: 0.0,
            end: 4.0,
        }, Oscilator::SINE);

        // send the buffer to the server in base64 encoding
        let response = client.post("/wav_data")
            .body(sine)
            .dispatch()
            .await;

        // check the response
        assert_eq!(response.status(), rocket::http::Status::Ok);
        // cast response body to a Chunk
        let chunk: Chunk = response.into_json().await.unwrap();
        // assert that the chunk contains only one note and that the note is A4
        assert_eq!(chunk.notes.len(), 1);
        assert_eq!(chunk.notes[0].pitch.name(), "A4");
        // assert that the start and end of the note are correct
        assert_eq!(chunk.notes[0].start, 0.0);
        assert_eq!(chunk.notes[0].end, 4.0);
        
    }
}
    