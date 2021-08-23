use rocket::{get, routes, Responder};
use thiserror::Error;
use tokio::{io, process::Command};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![spacer])
}

#[get("/model/spacer?<outer>&<inner>&<height>")]
async fn spacer(outer: f64, inner: f64, height: f64) -> Result<String, Error> {
    // TASK: Check status code.
    Command::new("openscad")
        .arg(format!("-Douter={}", outer))
        .arg(format!("-Dinner={}", inner))
        .arg(format!("-Dheight={}", height))
        // TASK: Store file in temporary directory.
        .args(["-o", "spacer.3mf"])
        .arg("api.scad")
        .current_dir("models/spacer")
        .status()
        .await?;

    // TASK: Serve generated file.
    Ok(format!("Call to OpenSCAD succeeded"))
}

#[derive(Debug, Error, Responder)]
enum Error {
    #[error("I/O error")]
    Io(#[from] io::Error),
}
