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
    let output = Command::new("openscad")
        .arg(format!("-Douter={}", outer))
        .arg(format!("-Dinner={}", inner))
        .arg(format!("-Dheight={}", height))
        // TASK: Store file in temporary directory.
        .args(["-o", "spacer.3mf"])
        .arg("api.scad")
        .current_dir("models/spacer")
        .output()
        .await?;

    // This can use `ExitStatus::exit_ok`, once that is stabilized.
    if !output.status.success() {
        return Err(Error::OpenScad(OpenScadError {
            stdout: output.stdout,
            stderr: output.stderr,
        }));
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
struct OpenScadError {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl<'r> RocketResponder<'r, 'static> for OpenScadError {
    fn respond_to(
        self,
        _: &'r rocket::Request,
    ) -> rocket::response::Result<'static> {
        let stdout = String::from_utf8(self.stdout)
            .unwrap_or_else(|err| format!("Error decoding stdout: {}", err));
        let stderr = String::from_utf8(self.stderr)
            .unwrap_or_else(|err| format!("Error decoding stderr: {}", err));

        error!(
            "Error calling OpenSCAD.\nstdout:\n{}\nstderr:\n{}",
            stdout, stderr
        );

        Err(Status::InternalServerError)
    }
}
