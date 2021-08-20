use rocket::{get, routes};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, spacer])
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/model/spacer?<outer>&<inner>&<height>")]
fn spacer(outer: f64, inner: f64, height: f64) -> String {
    format!("outer: {}, inner: {}, height: {}", outer, inner, height)
}
