use rocket::{
    error, get, http::Status, response::Responder as RocketResponder, routes,
    Responder,
};
use tempfile::tempdir;
use thiserror::Error;
use tokio::{fs::File, io, process::Command};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![spacer])
}

#[get("/models/spacer.3mf?<outer>&<inner>&<height>")]
async fn spacer(outer: f64, inner: f64, height: f64) -> Result<Model, Error> {
    let tmp = tempdir()?;
    let path = tmp.path().join("model.3mf");
    let path_str = path.as_os_str().to_str().ok_or(TempDirNotValidUtf8Error)?;

    let output = Command::new("openscad")
        .arg(format!("-Douter={}", outer))
        .arg(format!("-Dinner={}", inner))
        .arg(format!("-Dheight={}", height))
        .args(["-o", path_str])
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

    let file = File::open(path).await?;
    Ok(file.into())
}

#[derive(Responder)]
struct Model {
    inner: File,
    // TASK: Set caching headers.
}

impl From<File> for Model {
    fn from(inner: File) -> Self {
        Self { inner }
    }
}

// TASK: Add route that returns images of model.

#[derive(Debug, Error, Responder)]
enum Error {
    #[error("I/O error")]
    Io(#[from] io::Error),

    #[error("Temporary directory path is not valid UTF-8")]
    TempDirNotValidUtf8(#[from] TempDirNotValidUtf8Error),

    #[error("Error calling OpenSCAD")]
    OpenScad(OpenScadError),
}

#[derive(Debug, Error)]
#[error("Temporary directory path is not valid UTF-8")]
struct TempDirNotValidUtf8Error;

impl<'r> RocketResponder<'r, 'static> for TempDirNotValidUtf8Error {
    fn respond_to(
        self,
        _: &'r rocket::Request,
    ) -> rocket::response::Result<'static> {
        error!("Temporary directory path is not valid UTF-8",);
        Err(Status::InternalServerError)
    }
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
