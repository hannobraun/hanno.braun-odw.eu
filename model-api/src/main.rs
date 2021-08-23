use rocket::{
    error, get, http::Status, response::Responder as RocketResponder, routes,
    Responder,
};
use thiserror::Error;
use tokio::{io, process::Command};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![spacer])
}

#[get("/model/spacer?<outer>&<inner>&<height>")]
async fn spacer(outer: f64, inner: f64, height: f64) -> Result<String, Error> {
    let status = Command::new("openscad")
        .arg(format!("-Douter={}", outer))
        .arg(format!("-Dinner={}", inner))
        .arg(format!("-Dheight={}", height))
        // TASK: Store file in temporary directory.
        .args(["-o", "spacer.3mf"])
        .arg("api.scad")
        .current_dir("models/spacer")
        .status()
        .await?;

    // This can use `ExitStatus::exit_ok`, once that is stabilized.
    if !status.success() {
        // TASK: Add OpenSCAD output to error.
        return Err(Error::OpenScad(OpenScadError));
    }

    // TASK: Serve generated file.
    Ok(format!("Call to OpenSCAD succeeded"))
}

#[derive(Debug, Error, Responder)]
enum Error {
    #[error("I/O error")]
    Io(#[from] io::Error),

    #[error("Error calling OpenSCAD")]
    OpenScad(OpenScadError),
}

#[derive(Debug)]
struct OpenScadError;

impl<'r> RocketResponder<'r, 'static> for OpenScadError {
    fn respond_to(
        self,
        _: &'r rocket::Request,
    ) -> rocket::response::Result<'static> {
        error!("Error calling OpenSCAD");
        Err(Status::InternalServerError)
    }
}
