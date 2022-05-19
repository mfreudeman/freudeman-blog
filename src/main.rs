use std::{path::{Path, PathBuf}};
use rocket::fs::NamedFile;

extern crate pretty_env_logger;
//#[macro_use] extern crate log;
#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("public_html/index.html").to_path_buf()).await.ok()
}

#[get("/page/<name>")]
async fn page(name: &str) -> Option<NamedFile> {
    trace!("/page/{}", name);
    NamedFile::open(Path::new("public_html/pages/").join(format!("{}.html", name))).await.ok()
}

#[get("/static/<file..>")]
async fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public_html/static/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    pretty_env_logger::init();
    rocket::build().mount("/", routes![index, page, static_file])
}
