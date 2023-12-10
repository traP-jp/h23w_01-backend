#[macro_use]
extern crate rocket;

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![ping])
}
