use actix_web::Result;
use actix_files as fs;

use std::path::PathBuf;

pub async fn serve_external_frontend() -> Result<fs::NamedFile> {
    let path: PathBuf = PathBuf::from("./public/index.html");
    Ok(fs::NamedFile::open(path)?)
}