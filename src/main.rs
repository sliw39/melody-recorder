#[macro_use] extern crate rocket;
mod analysis;
mod seqdatastruct;
mod notes;
use std::io::Result;

use analysis::{Chunk, analyze_chunk};
use rocket::data::{ToByteUnit};
use rocket::serde::json::Json;
use rocket::tokio::io::AsyncReadExt;
use rocket::{post, data::Data};


#[derive(Responder)]
#[response(status = 418, content_type = "json")]
struct Response(&'static str);

#[get("/")]
fn index() -> Response {
    Response("Hello, world!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[post("/wav_data", data = "<data>")]
async fn receive_wav_data(data: Data<'_>) -> Result<Json<Chunk>> {
    let mut buffer = Vec::new();
    data.open(512.kibibytes()).read_to_end(&mut buffer).await?;

    let chunk = analyze_chunk(&buffer);
    Ok(Json(chunk))
}
/*
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    struct Node<'a> {
        name: String,
        previous: Option<&'a Node<'a>>,
        next: Option<&'a Node<'a>>,
    }

    #[test]
    fn test_liste_chainee() {
        let arr = vec!["1", "2", "3", "4", "5", "6"];
        let mut index = HashMap::new();
        let mut last_item = None;
        for (i, name) in arr.iter().enumerate() {
            let item = index.entry(i).or_insert(Node {
                name: name.to_owned().to_owned(),
                previous: last_item,
                next: None,
            });
            if let Some(last) = last_item {
                last.next = Some(item);
            }
            last_item = Some(item);
        }

        let start = index.get(&3).unwrap();
        let mut node = start;
        while let Some(prev) = node.previous {
            node = prev;
        }

        println!("{}", node.name); // "1"
    }
}
 */