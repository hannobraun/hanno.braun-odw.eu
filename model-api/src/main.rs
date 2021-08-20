use rocket::{get, routes};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}
