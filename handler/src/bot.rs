use async_trait::async_trait;
use rocket::data::{Data, FromData, Outcome, ToByteUnit};
use rocket::http::Status;
use rocket::request::Request;

use traq_bot_http::{Event, RequestParser};

pub struct BotEvent(pub Event);

#[async_trait]
impl<'a> FromData<'a> for BotEvent {
    type Error = ();

    async fn from_data(request: &'a Request<'_>, data: Data<'a>) -> Outcome<'a, Self, Self::Error> {
        let err = (Status::InternalServerError, ());
        let parser = match request.rocket().state::<RequestParser>() {
            Some(p) => p,
            None => {
                eprintln!("managed RequestParser not found");
                return Outcome::Error(err);
            }
        };
        let headers = request.headers();
        let headers: Vec<_> = headers.iter().collect();
        let capped_data = match data.open(8.megabytes()).into_bytes().await {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Error while reading request body: {}", e);
                return Outcome::Error(err);
            }
        };
        match parser.parse(headers.iter().map(|h| (h.name(), h.value())), &capped_data) {
            Ok(event) => Outcome::Success(BotEvent(event)),
            Err(e) => {
                eprintln!("Error while parsing request: {}", e);
                Outcome::Error(err)
            }
        }
    }
}

#[rocket::post("/", data = "<event>")]
pub async fn bot_event(event: BotEvent) -> Status {
    println!("event kind: {}", event.0.kind());
    // match event.0 {
    //     _ => (),
    // };
    Status::NoContent
}
